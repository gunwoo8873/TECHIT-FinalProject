use {
    actix_web::{get, web, App, HttpServer, Responder},
    actix_cors::Cors,
    sqlx::{PgPool, Error},
    std::{io::Result},
    crate::{
        server_env::{
            server_host, server_port
        },
        database_env::{
            database_host, database_port, database_user, database_password, database_name
        },
        api::{
            register_api::register_user,
            // signin_api::signin_user
        }
    },
};

#[get("/api/hello")]
async fn test_app() -> impl Responder {
    "Hello from Actix-Web!"
}

pub async fn _run() -> Result<()> {
    let db_host = database_host();
    let db_port = database_port();
    let db_user = database_user();
    let db_password = database_password();
    let db_name = database_name();

    let db_url = format!(
        "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
    );

    println!("DB Connect Successfully: {}", db_url);
    let pool =PgPool::connect(&db_url).await.expect("Failed to connect to the database");

    let host = server_host();
    let port = server_port();

    println!("Server Connect Successfully : http://{host}:{port}/api");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_origin("http://127.0.0.1:4000")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec!["Content-Type"])
                .max_age(3600))
            .app_data(web::Data::new(pool.clone()))
            .service(test_app)
            .service(register_user)
    })
        .bind((host, port))?
        .run()
        .await
}