// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use url::Url;

pub mod action;
pub mod priority;

pub use self::action::{Action, ActionType};
pub use self::priority::Priority;

/// JSON payload
///
/// <https://docs.ntfy.sh/publish/#publish-as-json>
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Payload {
    /// Target topic name (**required**).
    pub topic: String,
    /// Message body.
    pub message: Option<String>,
    /// Message title.
    pub title: Option<String>,
    /// List of tags that may or not map to emojis.
    pub tags: Option<Vec<String>>,
    /// Message priority
    pub priority: Option<Priority>,
    /// Custom user action buttons for notifications
    ///
    /// See <https://docs.ntfy.sh/publish/#action-buttons>
    pub actions: Option<Vec<Action>>,
    /// Website opened when notification is clicked
    pub click: Option<Url>,
    /// URL of an attachment, see attach via URL
    pub attach: Option<Url>,
    /// Set to `true` if the message is Markdown-formatted.
    pub markdown: Option<bool>,
    /// URL to use as notification icon.
    pub icon: Option<Url>,
    /// File name of the attachment.
    pub filename: Option<String>,
    /// Timestamp or duration for delayed delivery
    pub delay: Option<String>,
    /// E-mail address for e-mail notifications
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
            ..Default::default()
        }
    }

    /// Set message
    #[inline]
    pub fn message<S>(mut self, msg: S) -> Self
    where
        S: Into<String>,
    {
        self.message = Some(msg.into());
        self
    }

    /// Set title
    #[inline]
    pub fn title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    /// Set tags
    pub fn tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = Some(tags.into_iter().map(|t| t.into()).collect());
        self
    }

    /// Set priority
    #[inline]
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Set actions
    pub fn actions<I>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = Action>,
    {
        self.actions = Some(actions.into_iter().collect());
        self
    }

    /// Set click url
    #[inline]
    pub fn click(mut self, url: Url) -> Self {
        self.click = Some(url);
        self
    }

    /// Set attachment url
    #[inline]
    pub fn attach(mut self, url: Url) -> Self {
        self.attach = Some(url);
        self
    }

    /// Use markdown
    ///
    /// <https://docs.ntfy.sh/publish/#markdown-formatting>
    #[inline]
    pub fn markdown(mut self, markdown: bool) -> Self {
        self.markdown = Some(markdown);
        self
    }

    /// Set icon url
    #[inline]
    pub fn icon(mut self, url: Url) -> Self {
        self.icon = Some(url);
        self
    }

    /// Set filename
    #[inline]
    pub fn filename<S>(mut self, filename: S) -> Self
    where
        S: Into<String>,
    {
        self.filename = Some(filename.into());
        self
    }

    /// Set delay
    #[inline]
    pub fn delay<S>(mut self, delay: S) -> Self
    where
        S: Into<String>,
    {
        self.delay = Some(delay.into());
        self
    }

    /// Set email
    #[inline]
    pub fn email<S>(mut self, email: S) -> Self
    where
        S: Into<String>,
    {
        self.email = Some(email.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg(feature = "blocking-subscribing")]
pub enum ReceivedMessageType {
    Open,
    Keepalive,
    Message,
    MessageDelete,
    MessageClear,
    PollRequest,
}

/// JSON received attachment
///
/// <https://docs.ntfy.sh/subscribe/api/#json-message-format>
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg(feature = "blocking-subscribing")]
pub struct ReceivedAttachment {
    /// Name of the attachment.
    name: String,
    /// URL of the attachment.
    url: Url,
    /// Mime type of the attachment.
    #[serde(alias = "type")]
    mime_type: Option<String>,
    /// Size of the attachment in bytes.
    size: Option<u32>,
    /// Attachment expiry date as Unix time stamp.
    expires: Option<u32>,
}

/// JSON received payload
///
/// <https://docs.ntfy.sh/subscribe/api/#json-message-format>
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg(feature = "blocking-subscribing")]
pub struct ReceivedPayload {
    /// Randomly chosen message identifier.
    pub id: String,
    /// Message date time, as Unix time stamp.
    pub time: u32,
    /// Unix time stamp indicating when the message will be deleted,
    /// not set if Cache: no is sent
    pub expires: Option<u32>,
    /// Message type, typically you'd be only interested in message
    pub event: ReceivedMessageType,
    /// Comma-separated list of topics the message is associated with;
    /// only one for all message events, but may be a list in open events
    pub topic: String,
    /// Sequence ID for updating/deleting notifications
    pub sequence_id: Option<String>,
    /// Message body; always present in message events
    pub message: Option<String>,
    /// Message title; if not set defaults to ntfy.sh/<topic>
    pub title: Option<String>,
    /// List of tags that may or not map to emojis
    pub tags: Option<Vec<String>>,
    /// Message priority with 1=min, 3=default and 5=max
    pub priority: Option<Priority>,
    /// Website opened when notification is clicked
    pub click: Option<Url>,
    /// Action buttons that can be displayed in the notification
    pub actions: Option<Vec<Action>>,
    /// Details about an attachment (name, URL, size, ...)
    pub attachment: Option<ReceivedAttachment>,
}
