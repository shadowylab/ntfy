# Ntfy

[![crates.io](https://img.shields.io/crates/v/ntfy.svg)](https://crates.io/crates/ntfy)
[![Documentation](https://docs.rs/ntfy/badge.svg)](https://docs.rs/ntfy)
[![MIT](https://img.shields.io/crates/l/ntfy.svg)](LICENSE)

## Description

[Rust](https://rust-lang.org) library for sending notifications over [ntfy](https://ntfy.sh).

## Example

```rust,no_run
use ntfy::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let dispatcher = dispatcher::builder("https://ntfy.sh")
        .credentials(Auth::credentials("username", "password")) // Add optional credentials
        //.proxy("socks5h://127.0.0.1:9050") // Add optional proxy (requires "socks" feature)
        .build_async()?; // Build dispatcher

    let action = Action::new(
        ActionType::Http,
        "Turn down",
        Url::parse("https://api.nest.com")?,
    );

    let payload = Payload::new("mytopic")
        .message("Hello, **World**!") // Add optional message
        .title("Alert") // Add optiona title
        .tags(["warning"]) // Add optional tags
        .priority(Priority::High) // Edit priority
        .actions([action]) // Add optional actions
        .click(Url::parse("https://example.com")?) // Add optional clickable url
        .attach(Url::parse("https://example.com/file.jpg")?) // Add optional url attachment
        .delay("30min") // Add optional delay
        .markdown(true); // Use markdown

    dispatcher.send(&payload).await?;

    Ok(())
}
```

## Crate Feature Flags

The following crate feature flags are available:

| Feature      | Default | Description                        |
|--------------|:-------:|------------------------------------|
| `async`      |   Yes   | Use `reqwest` as dispatcher client |
| `blocking`   |   No    | Use `ureq` as dispatcher client    |
| `socks`      |   No    | Enables socks proxy support        |
| `rustls`     |   No    | Enables rust TLS                   |
| `native-tls` |   No    | Enables native TLS (openssl)       |

More examples can be found in the [examples](examples) directory.

## License

This project is distributed under the MIT software license - see the [LICENSE](LICENSE) file for details
