use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::PgPool;
use crate::entities::merchants::Merchant;
use crate::repositories::merchants::upsert_merchants;
use crate::events::handlers::handler::EventHandler;

pub struct MerchantUpsertedHandler {
    pool: PgPool,
}

impl MerchantUpsertedHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub struct MerchantUpsertedHandlerBuilder;

impl MerchantUpsertedHandlerBuilder {
    pub fn build(self, pool: PgPool) -> MerchantUpsertedHandler {
        MerchantUpsertedHandler::new(pool)
    }
}

#[async_trait]
impl EventHandler for MerchantUpsertedHandler {
    async fn handle(&self, payload: Value) -> Result<()> {

        let merchant = Merchant {
            id: uuid::Uuid::new_v4(),
            merchant_reference: payload["merchant_reference"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            live_on: chrono::NaiveDate::parse_from_str(
                payload["live_on"].as_str().unwrap_or_default(),
                "%Y-%m-%d",
            )?,
            disbursement_frequency: serde_json::from_value(payload["disbursement_frequency"].clone())?,
            minimum_monthly_fee: payload["minimum_monthly_fee"].as_i64().unwrap_or(0) as i32,
        };

        println!(
            "Processed merchant: {} | live_on: {} | fee: {}",
            merchant.merchant_reference,
            merchant.live_on,
            merchant.minimum_monthly_fee
        );

        upsert_merchants(&self.pool, &[merchant]).await?;

        Ok(())
    }
}
