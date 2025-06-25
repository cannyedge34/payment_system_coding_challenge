use sqlx::PgPool;

pub async fn clean_db(pool: &PgPool) {
  sqlx::query("TRUNCATE TABLE merchants RESTART IDENTITY CASCADE")
      .execute(pool)
      .await
      .unwrap();
}