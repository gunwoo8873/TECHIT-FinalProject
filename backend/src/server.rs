use actix_web::{web, App, HttpServer};
use std::{io::Result, env};

pub async fn run_server() -> Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Run server : http://{host}:{port}");

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/")
            )
    })
        .bind((host.as_str(), port))?
        .run()
        .await
}