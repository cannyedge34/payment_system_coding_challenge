use calculator::settings::config::Settings;
use calculator::events::handlers::merchant_upserted_handler::MerchantUpsertedHandlerBuilder;
use calculator::events::kafka::consumer::KafkaCalculatorConsumer;
use sqlx::PgPool;
use std::sync::Arc;
use anyhow::Result;
use calculator::events::handlers::handler::EventHandler;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let settings = Settings::from_env();
    let pool = PgPool::connect(&settings.database_url).await?;

    let handler = MerchantUpsertedHandlerBuilder.build(pool);
    let handler_arc: Arc<dyn EventHandler + Send + Sync> = Arc::new(handler);

    let consumer = KafkaCalculatorConsumer::new(
        handler_arc,
        &settings.kafka_brokers,
        "calculator-merchant-group",
    )?;

    consumer.subscribe(&["merchant_upserted"])?;

    consumer.run().await?;

    Ok(())
}
