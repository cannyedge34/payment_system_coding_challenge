use apalis::prelude::Job;
use crate::entities::merchants::Merchant;
use crate::events::publisher::EventPublisher;
use crate::repositories::merchants::upsert_merchants;
use sqlx::PgPool;
use crate::services::csv_normalizer::read_and_normalize_merchants_from_csv;
use crate::settings::config::Settings;

#[derive(Clone, Debug)]
pub struct ImportCsvJob {
    pub file_path: String,
}

impl Job for ImportCsvJob {
    const NAME: &'static str = "import-csv";
}

pub async fn import_merchants_csv_handler<P: EventPublisher>(job: ImportCsvJob, publisher: &P, settings: Settings) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let merchants = read_and_normalize_merchants_from_csv(&job.file_path)?;

    let pool = PgPool::connect(&settings.database_url).await?;

    let tx = pool.begin().await?;

    upsert_merchants(&pool, &merchants).await?;

    publish_merchants(publisher, merchants).await?;

    tx.commit().await?;

    Ok(())
}

async fn publish_merchants<P: EventPublisher>(
    publisher: &P,
    merchants: Vec<Merchant>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for merchant in &merchants {
        publisher.publish("merchant_upserted", serde_json::json!({
            // we do not want to send the "id" or "email" attributes to other services, in this case, the calculator service.
            "merchant_reference": merchant.merchant_reference,
            "live_on": merchant.live_on,
            "disbursement_frequency": merchant.disbursement_frequency,
            "minimum_monthly_fee": merchant.minimum_monthly_fee
        })).await?;
    }
    Ok(())
}
