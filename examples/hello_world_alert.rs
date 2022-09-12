// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use ntfy::{Dispatcher, Payload, Priority};

fn main() {
    let dispatcher = Dispatcher::new("https://ntfy.sh", Some("socks5h://127.0.0.1:9050")).unwrap();

    /* let payload = Payload {
        topic: String::from("mytopic"),
        message: String::from("Hello, World!"),
        priority: Priority::Default,
        title: Some(String::from("Alert")),
    }; */

    // or

    let payload = Payload::new("mytopic", "Hello, World!")
        .title("Alert") // Add optional title
        .priority(Priority::High); // Edit priority

    dispatcher.send(&payload).unwrap();
}
