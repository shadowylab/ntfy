// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#[cfg(feature = "blocking")]
use reqwest::blocking::Client as ReqwestClient;
#[cfg(not(feature = "blocking"))]
use reqwest::Client as ReqwestClient;
use url::Url;

pub mod auth;
pub mod builder;

pub use self::auth::Auth;
pub use self::builder::DispatcherBuilder;
use crate::error::NtfyError;
use crate::net::request;
use crate::payload::Payload;

#[derive(Clone)]
pub struct Dispatcher {
    url: Url,
    client: ReqwestClient,
}

impl Dispatcher {
    /// Create new dispatcher
    pub fn new<S>(url: S, auth: Option<Auth>, proxy: Option<&str>) -> Result<Self, NtfyError>
    where
        S: Into<String>,
    {
        let mut builder = DispatcherBuilder::new(url);

        if let Some(auth) = auth {
            builder = builder.credentials(auth);
        }

        if let Some(proxy) = proxy {
            builder = builder.proxy(proxy);
        }

        builder.build()
    }

    pub fn builder<S>(url: S) -> DispatcherBuilder
    where
        S: Into<String>,
    {
        DispatcherBuilder::new(url)
    }

    /// Send payload to ntfy server
    #[cfg(not(feature = "blocking"))]
    pub async fn send(&self, payload: &Payload) -> Result<(), NtfyError> {
        log::debug!("{:?}", payload);
        request(self.client.post(self.url.as_str()).json(payload)).await
    }

    /// Send payload to ntfy server
    #[cfg(feature = "blocking")]
    pub fn send(&self, payload: &Payload) -> Result<(), NtfyError> {
        log::debug!("{:?}", payload);
        request(self.client.post(self.url.as_str()).json(payload))
    }
}
