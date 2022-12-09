// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;

#[cfg(feature = "blocking")]
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
#[cfg(not(feature = "blocking"))]
use reqwest::ClientBuilder;
use reqwest::Proxy;
use url::Url;

use super::{Auth, Dispatcher};
use crate::error::NtfyError;

#[derive(Clone)]
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

    pub fn credentials(self, auth: Auth) -> Self {
        Self {
            auth: Some(auth),
            ..self
        }
    }

    pub fn proxy<S>(self, proxy: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            proxy: Some(proxy.into()),
            ..self
        }
    }

    pub fn build(self) -> Result<Dispatcher, NtfyError> {
        let mut client = ClientBuilder::new();

        if let Some(auth) = self.auth {
            let mut headers = HeaderMap::new();
            let mut auth_value = HeaderValue::from_str(&format!("Basic {}", auth.as_base64()))?;
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
}
