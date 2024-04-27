use std::str::FromStr;

use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::types::Uuid;

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

pub async fn is_logged_in(mut db: Connection<DB>, token: &str) -> bool {
    let token = Uuid::from_str(token);

    if let Err(_) = token {
        return false;
    }

    let token = token.unwrap();

    let query_result = sqlx::query!(
        r#"
            SELECT * FROM sessions WHERE id = $1
        "#,
        token
    )
    .fetch_one(&mut **db)
    .await;

    match query_result {
        Ok(e) => e.id == token,
        Err(_) => false,
    }
}
