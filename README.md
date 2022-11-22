# Ntfy SDK

## Description

[Ntfy](https://ntfy.sh) client library to send notifications from [Rust](https://rust-lang.org).

## Example

```toml
ntfy = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use ntfy::{Dispatcher, Payload, Priority, Auth};

#[tokio::main]
async fn main() {
    let auth = Auth::new("username", "password");
    let dispatcher = Dispatcher::new(
        "https://ntfy.sh",
        Some(auth),
        Some("socks5h://127.0.0.1:9050"),
    )
    .unwrap();

    let payload = Payload::new("mytopic", "Hello, World!")
        .title("Alert") // Add optional title
        .priority(Priority::High); // Edit priority

    dispatcher.send(&payload).await.unwrap();
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