use importer::jobs::import_csv::{ImportCsvJob, import_merchants_csv_handler};
use importer::settings::config::Settings;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
use chrono::NaiveDate;
mod utils;
mod mock_publisher;

use utils::clean_db;
use mock_publisher::MockPublisher;

#[tokio::test]
async fn it_imports_csv_and_insert_merchants_and_publish_event() {
    dotenvy::from_filename(".env.test").ok();
    let settings = Settings::from_env();
    let pool = PgPool::connect(&settings.database_url).await.unwrap();

    clean_db(&pool).await;
    let mock = MockPublisher::default();

    let job = ImportCsvJob { file_path: settings.csv_path.to_string() };
    let result = import_merchants_csv_handler(job, &mock, settings).await;

    let row = sqlx::query("SELECT COUNT(*) as count FROM merchants")
    .fetch_one(&pool)
    .await
    .unwrap();

    let count: i64 = row.get("count");
    assert_eq!(count, 4);

    assert!(result.is_ok());
    assert_eq!(mock.calls_count().await, 4);
    assert!(mock.was_called_with("merchant_upserted").await);
}
#[tokio::test]
async fn it_imports_csv_and_upsert_existing_merchants_and_publish_event() {
    let settings = Settings::from_env();
    let pool = PgPool::connect(&settings.database_url).await.unwrap();

    clean_db(&pool).await;

    let mock_publisher = MockPublisher::default();

    let existing_id = Uuid::parse_str("86312006-4d7e-45c4-9c28-788f4aa68a62").unwrap();
    let existing_email = "old_email@example.com";
    let existing_merchant_reference = "padberg_group";
    let existing_date = NaiveDate::parse_from_str("2022-01-01", "%Y-%m-%d").unwrap();
    let existing_frequency = "DAILY";
    let existing_min_fee = 10;

    sqlx::query(
        "INSERT INTO merchants (id, merchant_reference, email, live_on, disbursement_frequency, minimum_monthly_fee)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(existing_id)
    .bind(existing_merchant_reference)
    .bind(existing_email)
    .bind(existing_date)
    .bind(existing_frequency)
    .bind(existing_min_fee)
    .execute(&pool)
    .await
    .unwrap();

    let row_before = sqlx::query(
        "SELECT email, live_on, disbursement_frequency, minimum_monthly_fee
         FROM merchants WHERE merchant_reference = $1"
    )
    .bind(existing_merchant_reference)
    .fetch_one(&pool)
    .await
    .unwrap();

    let email_before: String = row_before.get("email");
    let live_on_before: NaiveDate = row_before.get("live_on");
    let frequency_before: String = row_before.get("disbursement_frequency");
    let min_fee_before: i32 = row_before.get("minimum_monthly_fee");

    assert_eq!(email_before, existing_email);
    assert_eq!(live_on_before, existing_date);
    assert_eq!(frequency_before, existing_frequency);
    assert_eq!(min_fee_before, existing_min_fee);

    let job = ImportCsvJob {
        file_path: settings.csv_path.to_string(),
    };
    let result = import_merchants_csv_handler(job, &mock_publisher, settings).await;

    let row = sqlx::query("SELECT COUNT(*) as count FROM merchants")
        .fetch_one(&pool)
        .await
        .unwrap();

    let count: i64 = row.get("count");
    assert_eq!(count, 4);

    let updated_row = sqlx::query(
        "SELECT email, live_on, disbursement_frequency, minimum_monthly_fee
         FROM merchants WHERE merchant_reference = $1"
    )
    .bind(existing_merchant_reference)
    .fetch_one(&pool)
    .await
    .unwrap();

    let updated_email: String = updated_row.get("email");
    let updated_live_on: NaiveDate = updated_row.get("live_on");
    let updated_frequency: String = updated_row.get("disbursement_frequency");
    let updated_min_fee: i32 = updated_row.get("minimum_monthly_fee");

    assert_eq!(updated_email, "info@padberg-group.com");
    assert_eq!(updated_live_on, NaiveDate::parse_from_str("2023-02-01", "%Y-%m-%d").unwrap());
    assert_eq!(updated_frequency, "DAILY");
    assert_eq!(updated_min_fee, 0);

    assert!(result.is_ok());
    assert_eq!(mock_publisher.calls_count().await, 4);
    assert!(mock_publisher.was_called_with("merchant_upserted").await);
}
