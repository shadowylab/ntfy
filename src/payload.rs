// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use crate::priority::{self, Priority};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub topic: String,
    pub message: String,
    #[serde(with = "priority")]
    pub priority: Priority,
    pub title: Option<String>,
}

impl Payload {
    /// Create new payload
    pub fn new(topic: &str, message: &str) -> Self {
        Self {
            topic: topic.into(),
            message: message.into(),
            priority: Priority::default(),
            title: None,
        }
    }

    /// Set priority
    pub fn priority(self, priority: Priority) -> Self {
        Self { priority, ..self }
    }

    /// Set title
    pub fn title(self, title: &str) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }
}
