// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use chrono::{DateTime, Local};
use url::Url;

pub mod action;
pub mod priority;

pub use self::action::{Action, ActionType};
pub use self::priority::Priority;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub topic: String,
    pub message: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(with = "priority")]
    pub priority: Priority,
    pub actions: Option<Vec<Action>>,
    pub click: Option<Url>,
    pub attach: Option<Url>,
    pub filename: Option<String>,
    pub delay: Option<String>,
    pub email: Option<String>,
}

impl Payload {
    /// Create new payload
    pub fn new<S>(topic: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            topic: topic.into(),
            message: String::new(),
            title: None,
            tags: None,
            priority: Priority::default(),
            actions: None,
            click: None,
            attach: None,
            filename: None,
            delay: None,
            email: None,
        }
    }

    /// Set message
    pub fn message<S>(self, msg: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message: msg.into(),
            ..self
        }
    }

    /// Set title
    pub fn title<S>(self, title: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    /// Set tags
    pub fn tags<V>(self, tags: V) -> Self
    where
        V: Into<Vec<String>>,
    {
        Self {
            tags: Some(tags.into()),
            ..self
        }
    }

    /// Set priority
    pub fn priority(self, priority: Priority) -> Self {
        Self { priority, ..self }
    }

    /// Set actions
    pub fn actions(self, actions: Vec<Action>) -> Self {
        Self {
            actions: Some(actions),
            ..self
        }
    }

    /// Set click url
    pub fn click(self, url: Url) -> Self {
        Self {
            click: Some(url),
            ..self
        }
    }

    /// Set attachment url
    pub fn attach(self, url: Url) -> Self {
        Self {
            attach: Some(url),
            ..self
        }
    }

    /// Set filename
    pub fn filename<S>(self, filename: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            filename: Some(filename.into()),
            ..self
        }
    }

    /// Set delay
    pub fn delay(self, time: DateTime<Local>) -> Self {
        Self {
            delay: Some(time.timestamp().to_string()),
            ..self
        }
    }

    /// Set email
    pub fn email<S>(self, email: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            email: Some(email.into()),
            ..self
        }
    }
}
