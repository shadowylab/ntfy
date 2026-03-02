use url::Url;

#[cfg(feature = "blocking-subscribing")]
use super::Blocking;
use super::{Error, Subscriber};

use crate::auth::Auth;

#[derive(Debug, Clone)]
pub struct SubscriberBuilder {
    url: String,
    pub(crate) auth: Option<Auth>,
}

impl SubscriberBuilder {
    #[inline]
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            url: url.into(),
            auth: None,
        }
    }

    #[inline]
    pub fn credentials(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    #[inline]
    pub fn proxy<S>(self, _proxy: S) -> Self
    where
        S: Into<String>,
    {
        panic!("Proxies are not supported by Subscriber.");
    }

    #[cfg(feature = "blocking-subscribing")]
    pub fn build_blocking(self) -> Result<Subscriber<Blocking>, Error> {
        Ok(Subscriber {
            url: Url::parse(&self.url)?,
            inner: Blocking::new(self)?,
        })
    }
}
