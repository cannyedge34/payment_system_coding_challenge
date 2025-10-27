use crate::events::handlers::handler::EventHandler;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use serde_json::Value;
use anyhow::Result;
use tokio_stream::StreamExt;
use std::sync::Arc;

const ENABLE_PARTITION_EOF: &str = "false";
const SESSION_TIMEOUT_MS: &str = "6000";
const ENABLE_AUTO_COMMIT: &str = "true";
const ENABLE_AUTO_OFFSET_RESET: &str = "earliest";

/// Kafka consumer that delegates messages to a handler
pub struct KafkaCalculatorConsumer {
    handler: Arc<dyn EventHandler + Send + Sync>,
    consumer: StreamConsumer,
}

impl KafkaCalculatorConsumer {
    pub fn new(
        handler: Arc<dyn EventHandler + Send + Sync>,
        brokers: &str,
        group_id: &str,
    ) -> Result<Self> {
        // Create the consumer with minimal config
        // In production, consider more flexible options via settings or a builder
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("enable.partition.eof", ENABLE_PARTITION_EOF)
            .set("session.timeout.ms", SESSION_TIMEOUT_MS)
            .set("enable.auto.commit", ENABLE_AUTO_COMMIT)
            .set("auto.offset.reset", ENABLE_AUTO_OFFSET_RESET)
            .create()?; // Will return error if broker is unreachable

        Ok(Self { handler, consumer })
    }

    pub fn subscribe(&self, topics: &[&str]) -> Result<()> {
        // Could add validation to ensure topics is not empty
        self.consumer.subscribe(topics)?;
        Ok(())
    }

    /// Run the consumer loop, processing messages indefinitely
    pub async fn run(&self) -> Result<()> {
        let mut stream = self.consumer.stream();

        // Infinite loop: consider graceful shutdown handling in production
        while let Some(message) = stream.next().await {
            match message {
                Ok(m) => {
                    if let Some(payload) = m.payload() {
                        match serde_json::from_slice::<Value>(payload) {
                            Ok(json) => {
                                // Delegate to the handler
                                // Could add retry/backoff or error tracking here
                                if let Err(err) = self.handler.handle(json).await {
                                    eprintln!("Error handling message: {:#}", err);
                                }
                            }
                            Err(err) => eprintln!("Failed to parse payload: {:#}", err),
                        }
                    }
                }
                Err(err) => {
                    // Could implement logging or reconnection/backoff here
                    eprintln!("Kafka error: {:#}", err);
                }
            }
        }

        Ok(())
    }
}
