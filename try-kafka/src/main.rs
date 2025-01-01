use std::sync::Arc;
use tokio;
use tokio::sync::Notify;

mod kafka;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let broker = "localhost:9092";
    let topic = "example";
    let group_id = "example_group";
    let notify = Arc::new(Notify::new());

    // kafka::produce_message(broker, topic, "example_key", "Hello, Kafka!").await;

    let consumer_notify = notify.clone();

    let consumer_task = tokio::spawn(async move {
        kafka::consume_messages(broker, topic, group_id, consumer_notify).await;
    });

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    println!("Shutting down...");

    notify.notify_one();

    consumer_task.await.expect("Failed to join consumer task");
}
