mod server;
mod api;

fn main() {
    server_run();
}

#[actix_web::main]
async fn server_run() {
    server::run_server().await.unwrap()
}