use ntfy::prelude::*;

fn main() -> Result<(), Error> {
    let subscriber = subscriber::builder("https://ntfy.sh")
        .credentials(Auth::credentials("username", "password")) // Add optional credentials
        .build_blocking()?; // Build subscriber

    // Subscribe to topic "test"
    let incoming_messages = subscriber.subscribe("mytopic")?;

    // Listen to 5 messages
    for message in incoming_messages.take(5) {
        match message {
            Ok(message) => println!("Received message: {message:?}"),
            Err(error) => println!("Error: {error}"),
        }
    }

    Ok(())
}
