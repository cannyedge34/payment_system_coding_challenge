use async_trait::async_trait;
use serde_json::Value;
use anyhow::Result;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, payload: Value) -> Result<()>;
}