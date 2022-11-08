# Ntfy SDK

## Description

[Ntfy](https://ntfy.sh) client library to send notifications from [Rust](https://rust-lang.org).

## Example

```toml
ntfy = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use ntfy::{Dispatcher, Payload, Priority};

#[tokio::main]
async fn main() {
    let dispatcher = Dispatcher::new("https://ntfy.sh", Some("socks5h://127.0.0.1:9050")).unwrap();

    let payload = Payload::new("mytopic", "Hello, World!")
        .title("Alert") // Add optional title
        .priority(Priority::High); // Edit priority

    dispatcher.send(&payload).await.unwrap();
}
```

## Blocking example

```toml
ntfy = { version = "0.1", features = ["blocking"] }
```

```rust,no_run
use ntfy::{Dispatcher, Payload, Priority};

fn main() {
    let dispatcher = Dispatcher::new("https://ntfy.sh", Some("socks5h://127.0.0.1:9050")).unwrap();

    let payload = Payload::new("mytopic", "Hello, World!")
        .title("Alert") // Add optional title
        .priority(Priority::High); // Edit priority

    dispatcher.send(&payload).unwrap();
}
```

## License

This project is distributed under the MIT software license - see the [LICENSE](LICENSE) file for details