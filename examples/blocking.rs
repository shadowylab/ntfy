// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use ntfy::prelude::*;

fn main() -> Result<(), NtfyError> {
    let dispatcher = Dispatcher::builder("https://ntfy.sh")
        .credentials(Auth::credentials("username", "password")) // Add optional credentials
        .proxy("socks5h://127.0.0.1:9050") // Add optional proxy
        .build()?; // Build dispatcher

    let action = Action::new(
        ActionType::Http,
        "Turn down",
        Url::parse("https://api.nest.com")?,
    );

    let payload = Payload::new("mytopic")
        .message("Hello, **World**!") // Add optional message
        .title("Alert") // Add optional title
        .tags(["warning"]) // Add optional tags
        .priority(Priority::High) // Edit priority
        .actions([action]) // Add optional actions
        .click(Url::parse("https://example.com")?) // Add optional clickable url
        .attach(Url::parse("https://example.com/file.jpg")?) // Add optional url attachment
        .delay(1639194738) // Add optional delay
        .markdown(true); // Use markdown

    dispatcher.send(&payload)?;

    Ok(())
}
