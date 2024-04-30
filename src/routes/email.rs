use super::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EmailInput {
    pub from: String,
    pub subject: String,
    pub body: String,
}

#[post("/email", data = "<email>", format = "json")]
pub async fn send_email<'a>(
    mut db: Connection<DB>,
    email: Json<EmailInput>,
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
            VALUES ($1, $2, $3, $4) RETURNING id
            "#,
        email.from,
        email.subject,
        email.body,
        ip.ip
    )
    .fetch_one(&mut **db)
    .await;

    if let Err(_) = query_result {
        return Err(BadRequest("Something went wrong on our end."));
    }

    let email_id = query_result.unwrap().id;

    // Create the email verification token using the email ID.
    let query_result = sqlx::query!(
        r#"
            INSERT into email_verification_tokens (token, email_id)
            VALUES ($1, $2)
            "#,
        email_id.to_string(),
        email_id
    )
    .execute(&mut **db)
    .await;

    if let Err(_) = query_result {
        return Err(BadRequest("Something went wrong on our end."));
    }

    println!("Email verification token: {}", email_id);

    Ok(format!(
        "Please verify your email address by clicking the link in the email sent to {}",
        email.from
    ))
}

#[get("/email")]
pub async fn get_emails(
    mut db: Connection<DB>,
    user: Auth,
) -> Result<Json<Vec<Email>>, BadRequest<&'static str>> {
    // Only allow a user to see the emails if they are logged in.
    if !user.is_logged_in {
        return Err(BadRequest("Unauthorized"));
    }

    let query_result = sqlx::query!(
        r#"
            SELECT source, subject, contents, ip_address, verified
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
            verified: row.verified.clone(),
        })
        .collect();

    Ok(Json(emails))
}
