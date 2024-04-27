use rocket::{
    request::{FromRequest, Outcome, Request},
    response::status::BadRequest,
    serde::{Deserialize, Serialize},
};
use rocket_db_pools::Connection;

use super::db::{is_logged_in, DB};

// This is a struct that will be used to store the IP address of the client.
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ipv4 {
    pub ip: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Ipv4 {
    type Error = BadRequest<&'static str>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(match req.client_ip() {
            Some(ip) => Self { ip: ip.to_string() },
            None => Self { ip: "".to_string() },
        })
    }
}

// This is a struct that will be used to store the email data.
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Email {
    pub from: String,
    pub subject: String,
    pub body: String,
}

// Struct for authentication
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Auth {
    pub is_logged_in: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = BadRequest<&'static str>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");
        let db = req.guard::<Connection<DB>>().await.unwrap();

        let unathorized = Outcome::Success(Self {
            is_logged_in: false,
        });

        match auth_header {
            Some(header) => {
                if !header.starts_with("Bearer ") {
                    return unathorized;
                }

                Outcome::Success(Self {
                    is_logged_in: is_logged_in(db, header.strip_prefix("Bearer ").unwrap()).await,
                })
            }
            None => unathorized,
        }
    }
}
