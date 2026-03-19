use futures_util::stream::StreamExt;
use ntfy::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = subscriber::builder("https://ntfy.sh")
        .credentials(Auth::credentials("username", "password")) // Add optional credentials
        .build_async()?; // Build subscriber

    // Subscribe to topic "test"
    let mut incoming_messages = subscriber.subscribe("mytopic").await?;

    // Listen
    while let Some(message) = incoming_messages.next().await {
        match message {
            Ok(message) => println!("Received message: {message:?}"),
            Err(error) => println!("Error: {error}"),
        }
    }

    Ok(())
}
