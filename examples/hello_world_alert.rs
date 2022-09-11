// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use ntfy::{Client, Payload, Priority};

fn main() {
    let client = Client::new("https://ntfy.sh", Some("socks5h://127.0.0.1:9050")).unwrap();

    let payload = Payload {
        topic: String::from("mytopic"),
        message: String::from("Hello, World!"),
        priority: Priority::Default,
        title: Some(String::from("Alert")),
    };

    client.publish(&payload).unwrap();
}
