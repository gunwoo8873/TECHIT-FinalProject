use sqlx::{PgPool, Error};
use dotenv::dotenv;
use std::env;

pub async fn local_db() -> Result<PgPool, Error> {
    dotenv().ok();

    let db_user = env::var("POSTGRES_USER").expect("DB_USER must be set");
    let db_pass = env::var("POSTGRES_PASSWORD").expect("DB_PASSWORD must be set");
    let db_host = env::var("POSTGRES_HOST").expect("DB_HOST must be set");
    let db_port = env::var("POSTGRES_PORT").expect("DB_PORT must be set");
    let db_name = env::var("POSTGRES_NAME").expect("DB_NAME must be set");

    let db_url = format!("
        postgres://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}
    ");
    // let db_url = env::var("POSTGRES_URL").expect("DATABASE_URL must be set");

    PgPool::connect(&db_url).await
}