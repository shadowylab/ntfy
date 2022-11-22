// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use serde_json::Value;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionType {
    View,
    Broadcast,
    Http,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub action: ActionType,
    pub label: String,
    pub url: Url,
    pub clear: Option<bool>,
    pub body: Option<Value>,
}

impl Action {
    /// Create new action
    pub fn new<S>(action: ActionType, label: S, url: Url) -> Self
    where
        S: Into<String>,
    {
        Self {
            action,
            label: label.into(),
            url,
            clear: None,
            body: None,
        }
    }

    /// Set clear
    pub fn clear(self, clear: bool) -> Self {
        Self {
            clear: Some(clear),
            ..self
        }
    }

    /// Set body
    pub fn body(self, body: Value) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }
}
