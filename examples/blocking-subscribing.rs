use ntfy::prelude::*;

fn main() -> Result<(), Error> {
    let subscriber = subscriber::builder("https://ntfy.sh")
        //.credentials(Auth::credentials("username", "password")) // Add optional credentials
        .build_blocking()?; // Build subscriber

    // Subscribe to topic "test"
    let incoming_messages = subscriber.subscribe("mytopic")?;

    // Listen
    for message in incoming_messages {
        match message {
            Ok(message) => println!("Received message: {message:?}"),
            Err(error) => println!("Error: {error}"),
        }
    }

    Ok(())
}
