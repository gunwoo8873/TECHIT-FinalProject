use backend::server::_run;

#[actix_web::main]
async fn main() {
    _run().await.unwrap()
}