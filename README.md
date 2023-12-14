# Ntfy SDK

[![crates.io](https://img.shields.io/crates/v/ntfy.svg)](https://crates.io/crates/ntfy)
[![Documentation](https://docs.rs/ntfy/badge.svg)](https://docs.rs/ntfy)
[![MIT](https://img.shields.io/crates/l/ntfy.svg)](LICENSE)

## Description

[Ntfy](https://ntfy.sh) client library to send notifications from [Rust](https://rust-lang.org).

## Example

```toml
chrono = "0.4"
ntfy = "0.4"
tokio = { version = "1", features = ["full"] }
url = "2"
```

```rust,no_run
use chrono::{Duration, Local};
use ntfy::payload::{Action, ActionType};
use ntfy::{Auth, Dispatcher, NtfyError, Payload, Priority};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), NtfyError> {
    let dispatcher = Dispatcher::builder("https://ntfy.sh")
        .credentials(Auth::new("username", "password")) // Add optional credentials
        .proxy("socks5h://127.0.0.1:9050") // Add optional proxy
        .build()?; // Build dispatcher

    let action = Action::new(
        ActionType::Http,
        "Turn down",
        Url::parse("https://api.nest.com")?,
    );

    let payload = Payload::new("mytopic")
        .message("Hello, **World**!") // Add optional message
        .title("Alert") // Add optiona title
        .tags(vec!["warning".into()]) // Add optional tags
        .priority(Priority::High) // Edit priority
        .actions([action]) // Add optional actions
        .click(Url::parse("https://example.com")?) // Add optional clickable url
        .attach(Url::parse("https://example.com/file.jpg")?) // Add optional url attachment
        .delay(Local::now() + Duration::minutes(1)) // Add optional delay
        .markdown(true); // Use markdown

    dispatcher.send(&payload).await.unwrap();

    Ok(())
}
```

More examples can be found in the [examples](./examples/) directory.

## Crate Feature Flags

The following crate feature flags are available:

| Feature             | Default | Description                                                                                                                |
| ------------------- | :-----: | -------------------------------------------------------------------------------------------------------------------------- |
| `blocking`          |   No    | Needed if you want to use this library in not async/await context                                                          |

## License

This project is distributed under the MIT software license - see the [LICENSE](LICENSE) file for details

## Donations

⚡ Tips: <https://getalby.com/p/yuki>

⚡ Lightning Address: yuki@getalby.com