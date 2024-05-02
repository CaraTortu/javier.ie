mod db;
mod mail_util;
mod routes;
mod structs;

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database;

use db::{run_migrations, DB};

use routes::email::*;
use routes::login::*;
use routes::verify::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("Run Migrations", run_migrations))
        .mount("/api", routes![send_email, get_emails, verify, login_post])
        .mount("/", FileServer::from(relative!("static/")))
}
