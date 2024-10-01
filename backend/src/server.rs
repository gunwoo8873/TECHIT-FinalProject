use actix_web::{web, App, HttpServer};
use std::{io::Result, env};
use dotenv::dotenv;
use crate::api;

pub async fn run_server() -> Result<()> {
    dotenv().ok();

    let db_pool = api::database::local_db()
        .await
        .expect("Failed to connect to the database");

    println!("DB Connect Successfully");

    let host = env::var("HOST")
        .expect("Host must be set");
        // .unwrap_or_else(|_| "127.0.0.1".to_string());

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Run server : http://{host}:{port}");

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::scope("/pages")
            )
    })
        .bind((host.as_str(), port))?
        .run()
        .await
}