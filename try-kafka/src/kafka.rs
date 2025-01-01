use std::sync::Arc;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tokio::sync::Notify;

pub async fn produce_message(broker: &str, topic: &str, key: &str, payload: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", broker)
        .create()
        .expect("Failed to create Kafka producer");

    let record = FutureRecord::to(topic).key(key).payload(payload);

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => println!("Delivered: {:?}", delivery),
        Err((e, _)) => eprintln!("Failed to deliver message: {}", e),
    }
}

pub async fn consume_messages(broker: &str, topic: &str, group_id: &str, notify: Arc<Notify>) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("group.id", group_id)
        .set("enable.auto.commit", "true")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer.subscribe(&[topic]).expect("Failed to subscribe to topic");

    println!("Waiting for messages...");
    loop {
        tokio::select! {
            _ = notify.notified() => {
                println!("Consumer shutting down...");
                break;
            }
            message = consumer.recv() => {
                match message {
                    Ok(message) => {
                        if let Some(Ok(payload)) = message.payload_view::<str>() {
                            println!("Received message: {}", payload);
                        } else {
                            println!("Received empty or invalid message");
                        }
                    }
                    Err(KafkaError::PartitionEOF(_)) => {
                        // Ignoring PartitionEOF errors
                    }
                    Err(e) => eprintln!("Error receiving message: {}", e),
                }
            }
        }
    }
}
