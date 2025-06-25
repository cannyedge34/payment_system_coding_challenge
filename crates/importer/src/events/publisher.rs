use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, topic: &str, payload: Value) -> Result<(), anyhow::Error>;
}
