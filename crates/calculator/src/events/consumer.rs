use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait EventConsumer: Send + Sync {
    async fn consume(&self, topic: &str, payload: Value) -> Result<(), anyhow::Error>;
}
