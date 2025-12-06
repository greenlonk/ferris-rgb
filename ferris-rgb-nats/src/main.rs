use bytes::Bytes;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // Connect to the NATS server
    let client = async_nats::connect("127.0.0.1:4222").await?;

    // Subscribe to the "messages" subject
    let mut subscriber = client.subscribe("messages").await?;

    // Publish messages to the "messages" subject
    for _ in 0..10 {
        client.publish("messages", "data".into()).await?;
    }

    // Receive and process messages
    while let Some(message) = subscriber.next().await {
        println!("Received message {:?}", message);
    }

    Ok(())
}
