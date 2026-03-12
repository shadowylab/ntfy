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
        S: Into<Option<String>>,
    {
        self.title = title.into();
        self
    }

    /// Set tags
    pub fn tags<I, S, P>(mut self, tags: P) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
        P: Into<Option<I>>,
    {
        self.tags = tags.into().map(|i| i.into_iter().map(Into::into).collect());
        self
    }

    /// Set priority
    #[inline]
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Set actions
    pub fn actions<I, P>(mut self, actions: P) -> Self
    where
        I: IntoIterator<Item = Action>,
        P: Into<Option<I>>,
    {
        self.actions = actions.into().map(|i| i.into_iter().collect());
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
        S: Into<Option<String>>,
    {
        self.filename = filename.into();
        self
    }

    /// Set delay
    #[inline]
    pub fn delay<S>(mut self, delay: S) -> Self
    where
        S: Into<Option<String>>,
    {
        self.delay = delay.into();
        self
    }

    /// Set email
    #[inline]
    pub fn email<S>(mut self, email: S) -> Self
    where
        S: Into<Option<String>>,
    {
        self.email = email.into();
        self
    }
}
