use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("javierie")]
pub struct DB(sqlx::PgPool);

// Runs the migrations on the database
pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = DB::fetch(&rocket).expect("Failed to get database connection");

    sqlx::migrate!("./migrations")
        .run(&**conn)
        .await
        .expect("Failed to run migrations");

    Ok(rocket)
}
