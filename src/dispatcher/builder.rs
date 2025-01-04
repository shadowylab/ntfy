// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;

#[cfg(not(feature = "blocking"))]
use reqwest::header::{HeaderMap, HeaderValue};
#[cfg(not(feature = "blocking"))]
use reqwest::ClientBuilder;
#[cfg(not(feature = "blocking"))]
use reqwest::Proxy;

//#[cfg(feature = "blocking")]


use url::Url;

use super::{Auth, Dispatcher};
use crate::error::NtfyError;

#[derive(Debug, Clone)]
pub struct DispatcherBuilder {
    url: String,
    auth: Option<Auth>,
    proxy: Option<String>,
}

impl DispatcherBuilder {
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            url: url.into(),
            auth: None,
            proxy: None,
        }
    }

    #[inline]
    pub fn credentials(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn proxy<S>(mut self, proxy: S) -> Self
    where
        S: Into<String>,
    {
        self.proxy = Some(proxy.into());
        self
    }

    #[cfg(not(feature = "blocking"))]
    pub fn build(self) -> Result<Dispatcher, NtfyError> {
        let mut client = ClientBuilder::new();

        if let Some(auth) = self.auth {
            let mut headers = HeaderMap::new();
            let mut auth_value = HeaderValue::from_str(&auth.to_header_value())?;
            auth_value.set_sensitive(true);
            headers.insert("Authorization", auth_value);
            client = client.default_headers(headers);
        }

        if let Some(proxy) = self.proxy {
            client = client.proxy(Proxy::all(proxy)?);
        }

        Ok(Dispatcher {
            url: Url::from_str(&self.url)?,
            client: client.build()?,
        })
    }

    #[cfg(feature = "blocking")]
    pub fn build(self) -> Result<Dispatcher, NtfyError> {
        use ureq::{Error, MiddlewareNext, Request, Response};

        let mut agent = ureq::builder();

        if let Some(auth) = self.auth {
            let heaver_value = auth.to_header_value();

            // Set the authorization headers of every request using a middleware function
            agent = agent.middleware(move|req: Request, next: MiddlewareNext| -> Result<Response, Error> {
                next.handle(req.set("Authorization", &heaver_value))
            });
        }

        if let Some(proxy) = self.proxy {
            let proxy = ureq::Proxy::new(proxy)?;
            agent = agent.proxy(proxy);
        }

        Ok(Dispatcher {
            url: Url::from_str(&self.url)?,
            agent: agent.build(),
        })
    }
}
