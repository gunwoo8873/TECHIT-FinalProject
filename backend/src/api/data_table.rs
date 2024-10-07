use {
    sqlx::PgPool,
    crate::{UserConfig},
};

pub async fn save_db(user_data: &UserConfig, pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        INSERT INTO users (user_name, user_id, user_email, user_password, user_phone)
        VALUES ($1, $2, $3, $4, $5);
    "#;

    sqlx::query(query)
        .bind(&user_data.user_name)
        .bind(&user_data.user_id)
        .bind(&user_data.user_email)
        .bind(&user_data.user_password)
        .bind(&user_data.user_phone)
        .execute(pool)
        .await?;

    Ok(())
}