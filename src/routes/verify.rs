use super::*;

#[get("/verify/<token>")]
pub async fn verify(
    mut db: Connection<DB>,
    token: &str,
) -> Result<&'static str, BadRequest<&'static str>> {
    let query_result = sqlx::query!(
        r#"
            SELECT * FROM email_verification_tokens
            WHERE token = $1
        "#,
        token
    )
    .fetch_one(&mut **db)
    .await;

    if let Err(_) = query_result {
        return Err(BadRequest("Invalid token or you have already used it"));
    }

    let email_id = query_result.unwrap().email_id;

    let query_result = sqlx::query!(
        r#"
            UPDATE emails
            SET verified = true
            WHERE id = $1
        "#,
        email_id
    )
    .execute(&mut **db)
    .await;

    if let Err(_) = query_result {
        return Err(BadRequest("Something went wrong on our end."));
    }

    let query_result = sqlx::query!(
        r#"
            DELETE FROM email_verification_tokens
            WHERE token = $1
        "#,
        token
    )
    .execute(&mut **db)
    .await;

    if let Err(_) = query_result {
        return Err(BadRequest("Something went wrong on our end."));
    }

    Ok("Email verified")
}
