use url::Url;

#[cfg(feature = "blocking-subscribing")]
mod blocking;
pub mod builder;

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
