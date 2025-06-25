use std::sync::Arc;
use async_trait::async_trait;
use importer::events::publisher::EventPublisher;
use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct MockPublisher {
    pub published: Arc<Mutex<Vec<(String, Value)>>>,
}

#[async_trait]
impl EventPublisher for MockPublisher {
    async fn publish(&self, topic: &str, payload: Value) -> Result<(), anyhow::Error> {
        let mut lock = self.published.lock().await;
        lock.push((topic.to_string(), payload));
        Ok(())
    }
}

impl MockPublisher {
    pub async fn was_called_with(&self, topic: &str) -> bool {
        let lock = self.published.lock().await;
        lock.iter().any(|(t, _)| t == topic)
    }

    pub async fn calls_count(&self) -> usize {
        let lock = self.published.lock().await;
        lock.len()
    }
}
