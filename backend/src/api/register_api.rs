use {
    actix_web::{post, web, HttpResponse},
    sqlx::PgPool,
    crate::{
        UserConfig,
        api::data_table::save_db
    },
};

#[post("/api/register")]
async fn register_user(user_data: web::Json<UserConfig>, pool: web::Data<PgPool>) -> HttpResponse {
    // DB에 데이터 저장 로직
    let result = save_db(&user_data, &pool).await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}