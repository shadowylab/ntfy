use futures::stream::StreamExt;

use ntfy::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = subscriber::builder("https://ntfy.sh")
        .credentials(Auth::credentials("username", "password")) // Add optional credentials
        .build_async()?; // Build subscriber

    // Subscribe to topic "test"
    let incoming_messages = subscriber.subscribe("mytopic").await?;

    // Listen to 5 messages
    let mut five_next_messages = incoming_messages.take(5);
    while let Some(message) = five_next_messages.next().await {
        match message {
            Ok(message) => println!("Received message: {message:?}"),
            Err(error) => println!("Error: {error}"),
        }
    }

    Ok(())
}
