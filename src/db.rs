use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};

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
    let query_result = sqlx::query!(
        r#"
            SELECT * FROM sessions WHERE token = $1
        "#,
        token
    )
    .fetch_one(&mut **db)
    .await;

    match query_result {
        Ok(e) => e.token == token,
        Err(_) => false,
    }
}
