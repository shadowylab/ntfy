// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use url::Url;

pub mod action;
pub mod priority;

pub use self::action::{Action, ActionType};
pub use self::priority::Priority;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Payload {
    pub topic: String,
    pub message: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub priority: Priority,
    pub actions: Option<Vec<Action>>,
    pub click: Option<Url>,
    pub attach: Option<Url>,
    pub filename: Option<String>,
    pub delay: Option<String>,
    pub email: Option<String>,
    pub icon: Option<Url>,
    #[serde(skip)]
    pub markdown: bool,
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
        self.message = msg.into();
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
        self.priority = priority;
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

    /// Set icon url
    #[inline]
    pub fn icon(mut self, url: Url) -> Self {
        self.icon = Some(url);
        self
    }

    /// Use markdown
    ///
    /// <https://docs.ntfy.sh/publish/#markdown-formatting>
    #[inline]
    pub fn markdown(mut self, markdown: bool) -> Self {
        self.markdown = markdown;
        self
    }
}
