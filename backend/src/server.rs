use actix_web::{web, App, HttpServer};
use std::io::Result;

pub async fn run_server() -> Result<()> {
    let port = 5000;
    let host = "127.0.0.1";

    println!("Run server : http://127.0.0.1:5000");

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
            )
    })
        .bind((host, port))?
        .run()
        .await
}