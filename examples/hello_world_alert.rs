// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use ntfy::{Auth, Dispatcher, NtfyError, Payload, Priority};

#[tokio::main]
async fn main() -> Result<(), NtfyError> {
    let dispatcher = Dispatcher::builder("https://ntfy.sh")
        .credentials(Auth::new("username", "password")) // Add optional credentials
        .proxy("socks5h://127.0.0.1:9050") // Add optional proxy
        .build()?; // Build dispatcher

    let payload = Payload::new("mytopic", "Hello, World!")
        .title("Alert") // Add optional title
        .priority(Priority::High); // Edit priority

    dispatcher.send(&payload).await.unwrap();

    Ok(())
}
