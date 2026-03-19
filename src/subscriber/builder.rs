use url::Url;

#[cfg(feature = "async-subscriber")]
use super::Async;
#[cfg(feature = "blocking-subscriber")]
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

    #[cfg(feature = "async-subscriber")]
    pub fn build_async(self) -> Result<Subscriber<Async>, Error> {
        let mut url: Url = Url::parse(&self.url)?;

        change_scheme(&mut url)?;

        Ok(Subscriber {
            url,
            inner: Async::new(self)?,
        })
    }

    #[cfg(feature = "blocking-subscriber")]
    pub fn build_blocking(self) -> Result<Subscriber<Blocking>, Error> {
        let mut url: Url = Url::parse(&self.url)?;

        change_scheme(&mut url)?;

        Ok(Subscriber {
            url,
            inner: Blocking::new(self)?,
        })
    }
}

fn change_scheme(url: &mut Url) -> Result<(), Error> {
    // Change scheme
    match url.scheme() {
        "https" | "wss" => url.set_scheme("wss").expect("Valid scheme"),
        "http" | "ws" => url.set_scheme("ws").expect("Valid scheme"),
        other => return Err(Error::InvalidScheme(other.to_string())),
    };

    Ok(())
}
