use rocket::{response::status::BadRequest, serde::json::Json};
use rocket_db_pools::Connection;

use regex::Regex;
use rocket::{
    http::Status,
    response::{self, Responder},
    serde::{Deserialize, Serialize},
    Response,
};
use sqlx::types::Uuid;

use super::db::DB;
use super::structs::{Auth, Email, Ipv4};

pub mod email;
pub mod login;
pub mod verify;
