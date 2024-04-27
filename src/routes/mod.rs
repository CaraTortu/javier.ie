use rocket::{response::status::BadRequest, serde::json::Json};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use regex::Regex;

use super::db::DB;
use super::structs::{Auth, Email, Ipv4};

pub mod email;
pub mod index;
pub mod verify;
