// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#[cfg(feature = "async-dispatcher")]
use reqwest::ClientBuilder;
#[cfg(feature = "blocking-dispatcher")]
use ureq::config::ConfigBuilder;
#[cfg(feature = "blocking-dispatcher")]
use ureq::typestate::AgentScope;
#[cfg(any(feature = "async-dispatcher", feature = "blocking-dispatcher"))]
use url::Url;

#[cfg(feature = "async-dispatcher")]
use super::Async;
#[cfg(feature = "blocking-dispatcher")]
use super::Blocking;
#[cfg(any(feature = "async-dispatcher", feature = "blocking-dispatcher"))]
use super::{Dispatcher, Error};
use crate::auth::Auth;

#[derive(Debug, Clone)]
pub struct DispatcherBuilder {
    url: String,
    pub(crate) auth: Option<Auth>,
    #[cfg(all(feature = "socks", not(target_arch = "wasm32")))]
    pub(crate) proxy: Option<String>,
}

impl DispatcherBuilder {
    #[inline]
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            url: url.into(),
            auth: None,
            #[cfg(all(feature = "socks", not(target_arch = "wasm32")))]
            proxy: None,
        }
    }

    #[inline]
    pub fn credentials(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    #[inline]
    #[cfg(all(feature = "socks", not(target_arch = "wasm32")))]
    pub fn proxy<S>(mut self, proxy: S) -> Self
    where
        S: Into<Option<String>>,
    {
        self.proxy = proxy.into();
        self
    }

    #[cfg(feature = "async-dispatcher")]
    #[deprecated(
        since = "0.7.0",
        note = "Please use `build_async` or `build_blocking` instead"
    )]
    pub fn build(self) -> Result<Dispatcher<Async>, Error> {
        self.build_async()
    }

    #[cfg(feature = "async-dispatcher")]
    pub fn build_async(self) -> Result<Dispatcher<Async>, Error> {
        Ok(Dispatcher {
            url: Url::parse(&self.url)?,
            inner: Async::new(self)?,
        })
    }

    #[cfg(feature = "async-dispatcher")]
    pub fn build_async_with_client(
        self,
        client: ClientBuilder,
    ) -> Result<Dispatcher<Async>, Error> {
        Ok(Dispatcher {
            url: Url::parse(&self.url)?,
            inner: Async::new_with_client(self, client)?,
        })
    }

    #[cfg(feature = "blocking-dispatcher")]
    pub fn build_blocking(self) -> Result<Dispatcher<Blocking>, Error> {
        Ok(Dispatcher {
            url: Url::parse(&self.url)?,
            inner: Blocking::new(self)?,
        })
    }

    #[cfg(feature = "blocking-dispatcher")]
    pub fn build_blocking_with_client(
        self,
        client: ConfigBuilder<AgentScope>,
    ) -> Result<Dispatcher<Blocking>, Error> {
        Ok(Dispatcher {
            url: Url::parse(&self.url)?,
            inner: Blocking::new_with_client(self, client)?,
        })
    }
}
