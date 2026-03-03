use url::Url;

#[cfg(feature = "async-subscribing")]
mod r#async;
#[cfg(feature = "blocking-subscribing")]
mod blocking;
pub mod builder;
mod request;

#[cfg(feature = "async-subscribing")]
pub use self::r#async::{Async, MessageStream};
#[cfg(feature = "blocking-subscribing")]
pub use self::blocking::{Blocking, MessageIterator};
pub use self::builder::SubscriberBuilder;
use crate::error::Error;

/// Creates a [`SubscriberBuilder`]
#[inline]
pub fn builder<S>(url: S) -> SubscriberBuilder
where
    S: Into<String>,
{
    SubscriberBuilder::new(url)
}

#[derive(Debug, Clone)]
pub struct Subscriber<T>
where
    T: Clone,
{
    url: Url,
    inner: T,
}

#[cfg(feature = "async-subscribing")]
impl Subscriber<Async> {
    /// Subscribe to ntfy server topic
    #[inline]
    pub async fn subscribe<S>(&self, topic: S) -> Result<MessageStream, Error>
    where
        S: Into<String>,
    {
        self.inner.subscribe(&self.url, topic.into()).await
    }
}

#[cfg(feature = "blocking-subscribing")]
impl Subscriber<Blocking> {
    /// Subscribe to ntfy server topic
    #[inline]
    pub fn subscribe<S>(&self, topic: S) -> Result<MessageIterator, Error>
    where
        S: Into<String>,
    {
        self.inner.subscribe(&self.url, topic.into())
    }
}
