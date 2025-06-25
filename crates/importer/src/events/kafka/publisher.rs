use crate::events::publisher::EventPublisher;
use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_json::Value;
use std::time::Duration;

pub struct KafkaPublisher {
    producer: FutureProducer,
}

pub struct KafkaPublisherBuilder {
    brokers: String,
}

/*
    In the initializer, I only create the empty/configured instance, and then in another (build).
    more explicit method (connect, create_brokers, etc.). I handle the parts that can fail.
    This is a very common practice in Ruby, Python, etc., and it has its advantages:
    Initializers (initialize, new) must be fast and safe.
    Failures, such as connections, network, etc., go into explicit methods (more predictable and testable).
*/
impl KafkaPublisherBuilder {
    // This constructor should not fail
    pub fn new(brokers: &str) -> Self {
        Self {
            brokers: brokers.to_string(),
        }
    }

    // This build can fail
    pub fn build(self) -> Result<KafkaPublisher, anyhow::Error> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .create()?;
        Ok(KafkaPublisher { producer })
    }
}


#[async_trait]
impl EventPublisher for KafkaPublisher {
    async fn publish(&self, topic: &str, payload: Value) -> Result<(), anyhow::Error> {
        let payload_str = serde_json::to_string(&payload)?;

        let key = payload.get("data")
            .and_then(|data| data.get("reference"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let record = FutureRecord::<_, _>::to(topic)
            .payload(&payload_str)
            .key(key);

        match self.producer.send(record, Duration::ZERO).await {
            Ok(_) => Ok(()),
            Err((err, _message)) => Err(anyhow::Error::new(err))
        }
    }
}