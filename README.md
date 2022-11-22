# Ntfy SDK

## Description

[Ntfy](https://ntfy.sh) client library to send notifications from [Rust](https://rust-lang.org).

## Example

```toml
ntfy = "0.2"
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
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
```

More examples can be found in the [examples](./examples/) directory.

## Crate Feature Flags

The following crate feature flags are available:

| Feature             | Default | Description                                                                                                                |
| ------------------- | :-----: | -------------------------------------------------------------------------------------------------------------------------- |
| `blocking`          |   No    | Needed if you want to use this library in not async/await context                                                          |

## License

This project is distributed under the MIT software license - see the [LICENSE](LICENSE) file for details