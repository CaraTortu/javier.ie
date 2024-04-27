mod db;
mod routes;
mod structs;

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

use db::{run_migrations, DB};
use routes::email::{get_emails, send_email};
use routes::index::index;
use routes::verify::verify;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DB::init())
        .attach(AdHoc::try_on_ignite("Run Migrations", run_migrations))
        .attach(Template::fairing())
        .mount("/", routes![index, send_email, get_emails, verify])
        .mount("/static", FileServer::from(relative!("static")))
}
