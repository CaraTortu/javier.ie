#[macro_use]
extern crate rocket;

use rocket::{
    fairing::AdHoc,
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
    response::status::BadRequest,
    serde::{json::Json, Deserialize, Serialize},
};

use rocket_db_pools::{Connection, Database};

mod db;
use db::{run_migrations, DB};
use regex::Regex;

struct Ipv4 {
    ip: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Ipv4 {
    type Error = BadRequest<&'static str>;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let ip = req.client_ip().map(|ip| ip.to_string());
        match ip {
            Some(ip) => Outcome::Success(Ipv4 { ip }),
            None => Outcome::Error((Status::BadRequest, BadRequest("No IP address found"))),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Email {
    from: String,
    subject: String,
    body: String,
    from_ip: String,
}

#[post("/send_email", data = "<email>", format = "json")]
async fn send_email<'a>(
    mut db: Connection<DB>,
    email: Json<Email>,
    ip: Ipv4,
) -> Result<String, BadRequest<&'static str>> {
    // Verify that no fields are empty
    if email.from.is_empty() || email.subject.is_empty() || email.body.is_empty() {
        return Err(BadRequest("All fields are required"));
    }

    // Verify that the email address is valid through REGEX
    let email_verify = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    if !email_verify.is_match(&email.from) {
        return Err(BadRequest("Invalid email address"));
    }

    let query_result = sqlx::query!(
        r#"
            INSERT INTO emails (source, subject, contents, ip_address)
            VALUES ($1, $2, $3, $4)
            "#,
        email.from,
        email.subject,
        email.body,
        ip.ip
    )
    .execute(&mut **db)
    .await;

    if let Err(_) = query_result {
        return Err(BadRequest("Failed to insert email"));
    }

    Ok(format!(
        "Sending email from {} with subject: {}",
        email.from, email.subject
    ))
}

#[get("/emails")]
async fn get_emails(mut db: Connection<DB>) -> Result<Json<Vec<Email>>, BadRequest<&'static str>> {
    let query_result = sqlx::query!(
        r#"
            SELECT source, subject, contents, ip_address
            FROM emails
            "#,
    )
    .fetch_all(&mut **db)
    .await;

    if let Err(_) = query_result {
        return Err(BadRequest("Failed to fetch emails"));
    }
    let res = query_result.unwrap();
    if res.is_empty() {
        return Err(BadRequest("No emails found"));
    }

    let emails = res
        .iter()
        .map(|row| Email {
            from: row.source.clone(),
            subject: row.subject.clone(),
            body: row.contents.clone(),
            from_ip: row.ip_address.clone(),
        })
        .collect();

    Ok(Json(emails))
}

#[get("/")]
async fn index() -> &'static str {
    "hello"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("Run Migrations", run_migrations))
        .mount("/", routes![index, send_email, get_emails])
}
