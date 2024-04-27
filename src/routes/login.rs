use super::*;

#[get("/login")]
pub async fn login() -> Template {
    Template::render("login", context! {})
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AuthLogin {
    email: String,
    password: String,
}

pub struct AuthLoginResponse {
    pub session: Uuid,
}

impl<'r> Responder<'r, 'static> for AuthLoginResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status::SeeOther)
            .header(rocket::http::Header::new("Location", "/email"))
            .header(rocket::http::Header::new(
                "Set-Cookie",
                format!("session={}", self.session),
            ))
            .ok()
    }
}

#[post("/login", data = "<auth>", format = "json")]
pub async fn login_post(
    auth: Json<AuthLogin>,
    mut db: Connection<DB>,
) -> Option<AuthLoginResponse> {
    let email = sqlx::query!(
        r#"
        SELECT * FROM users WHERE email = $1 AND password = $2
        "#,
        auth.email,
        auth.password
    )
    .fetch_one(&mut **db)
    .await;

    match email {
        Ok(email) => {
            let query_result = sqlx::query!(
                r#"
                INSERT INTO sessions (user_id) VALUES ($1) returning id
                "#,
                email.id
            )
            .fetch_one(&mut **db)
            .await
            .unwrap();

            Some(AuthLoginResponse {
                session: query_result.id,
            })
        }
        Err(_) => None,
    }
}
