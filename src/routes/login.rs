use sha256::digest;

use super::*;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AuthLogin {
    email: String,
    password: String,
}

pub struct AuthLoginResponse {
    pub session: Uuid,
    pub status: Status,
    pub location: String,
}

impl<'r> Responder<'r, 'static> for AuthLoginResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .header(rocket::http::Header::new("Location", self.location))
            .header(rocket::http::Header::new(
                "Set-Cookie",
                format!("session={}", self.session),
            ))
            .ok()
    }
}

#[post("/login", data = "<auth>", format = "json")]
pub async fn login_post(auth: Json<AuthLogin>, mut db: Connection<DB>) -> AuthLoginResponse {
    let password = digest(auth.password.to_string());

    let email = sqlx::query!(
        r#"
        SELECT * FROM users WHERE email = $1 AND password = $2
        "#,
        auth.email,
        password
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

            AuthLoginResponse {
                session: query_result.id,
                status: Status::SeeOther,
                location: "/mail".to_string(),
            }
        }
        Err(_) => AuthLoginResponse {
            session: Uuid::nil(),
            status: Status::Unauthorized,
            location: "/login".to_string(),
        },
    }
}
