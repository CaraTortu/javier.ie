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

    let email_fields = sqlx::query!(
        r#"
            UPDATE emails
            SET verified = true
            WHERE id = $1 
            RETURNING *
        "#,
        email_id
    )
    .fetch_one(&mut **db)
    .await;

    if let Err(_) = email_fields {
        return Err(BadRequest("Something went wrong on our end."));
    }
    /*
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
    */
    let email_fields = email_fields.unwrap();
    let from = email_fields.source;
    let subject = email_fields.subject;
    let body = email_fields.contents;
    let ip = email_fields.ip_address;

    let res = send_email(&from, &subject, &body, &ip).await;

    if let Err(_) = res {
        return Err(BadRequest("Failed to send email"));
    }

    Ok("Email verified")
}
