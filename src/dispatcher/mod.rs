// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use url::Url;

#[cfg(feature = "async")]
mod r#async;
pub mod auth;
#[cfg(feature = "blocking")]
mod blocking;
pub mod builder;

pub use self::auth::Auth;
#[cfg(feature = "blocking")]
pub use self::blocking::Blocking;
pub use self::builder::DispatcherBuilder;
#[cfg(feature = "async")]
pub use self::r#async::Async;
use crate::error::Error;
#[cfg(any(feature = "async", feature = "blocking"))]
use crate::payload::Payload;

/// Creates a [`DispatcherBuilder`]
#[inline]
pub fn builder<S>(url: S) -> DispatcherBuilder
where
    S: Into<String>,
{
    DispatcherBuilder::new(url)
}

#[derive(Debug, Clone)]
pub struct Dispatcher<T>
where
    T: Clone,
{
    url: Url,
    inner: T,
}

impl<T> Dispatcher<T>
where
    T: Clone,
{
    /// Create new dispatcher
    #[deprecated(since = "0.7.0", note = "Use the `DispatcherBuilder` instead")]
    pub fn new<S>(_url: S, _auth: Option<Auth>, _proxy: Option<S>) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        unimplemented!()
    }

    #[deprecated(
        since = "0.7.0",
        note = "Use `dispatcher::builder` or `DispatcherBuilder::new` instead"
    )]
    pub fn builder<S>(url: S) -> DispatcherBuilder
    where
        S: Into<String>,
    {
        DispatcherBuilder::new(url)
    }
}

#[cfg(feature = "async")]
impl Dispatcher<Async> {
    /// Send payload to ntfy server
    #[inline]
    pub async fn send(&self, payload: &Payload) -> Result<(), Error> {
        self.inner.send(&self.url, payload).await
    }
}

#[cfg(feature = "blocking")]
impl Dispatcher<Blocking> {
    /// Send payload to ntfy server
    #[inline]
    pub fn send(&self, payload: &Payload) -> Result<(), Error> {
        self.inner.send(&self.url, payload)
    }
}
