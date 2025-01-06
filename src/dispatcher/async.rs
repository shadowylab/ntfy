// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use reqwest::header::{HeaderMap, HeaderValue};
#[cfg(not(target_arch = "wasm32"))]
use reqwest::Proxy;
use reqwest::{Client, ClientBuilder, Response};
use url::Url;

use super::builder::DispatcherBuilder;
use crate::error::Error;
use crate::payload::Payload;

/// Async dispatcher
#[derive(Debug, Clone)]
pub struct Async {
    client: Client,
}

impl Async {
    #[inline]
    pub(crate) fn new(builder: DispatcherBuilder) -> Result<Self, Error> {
        Self::new_with_client(builder, ClientBuilder::new())
    }

    pub(crate) fn new_with_client(
        builder: DispatcherBuilder,
        mut client: ClientBuilder,
    ) -> Result<Self, Error> {
        if let Some(auth) = builder.auth {
            let mut headers = HeaderMap::new();
            let mut auth_value = HeaderValue::from_str(&auth.to_header_value())?;
            auth_value.set_sensitive(true);
            headers.insert("Authorization", auth_value);
            client = client.default_headers(headers);
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(proxy) = builder.proxy {
            client = client.proxy(Proxy::all(proxy)?);
        }

        Ok(Self {
            client: client.build()?,
        })
    }

    /// Send payload to ntfy server
    pub(crate) async fn send(&self, url: &Url, payload: &Payload) -> Result<(), Error> {
        // Build request
        let mut builder = self.client.post(url.as_str());

        // If markdown, set headers
        if payload.markdown {
            builder = builder.header("Markdown", "yes");
        }

        // Add payload
        builder = builder.json(payload);

        // Send request
        let res: Response = builder.send().await?;
        let res: Response = res.error_for_status()?;

        // Get full response text
        let text: String = res.text().await?;

        if text.is_empty() {
            return Err(Error::EmptyResponse);
        }

        // TODO: check the text?

        Ok(())
    }
}
