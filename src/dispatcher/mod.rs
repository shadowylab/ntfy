// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#[cfg(not(feature = "blocking"))]
use reqwest::{Client, Response};
#[cfg(feature = "blocking")]
use ureq::{Agent, Response};
use url::Url;

pub mod auth;
pub mod builder;

pub use self::auth::Auth;
pub use self::builder::DispatcherBuilder;
use crate::error::NtfyError;
use crate::payload::Payload;

#[derive(Debug, Clone)]
pub struct Dispatcher {
    url: Url,
    #[cfg(not(feature = "blocking"))]
    client: Client,
    #[cfg(feature = "blocking")]
    agent: Agent,
}

impl Dispatcher {
    /// Create new dispatcher
    pub fn new<S>(url: S, auth: Option<Auth>, proxy: Option<S>) -> Result<Self, NtfyError>
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

    #[inline]
    pub fn builder<S>(url: S) -> DispatcherBuilder
    where
        S: Into<String>,
    {
        DispatcherBuilder::new(url)
    }

    #[cfg(not(feature = "blocking"))]
    /// Send payload to ntfy server
    pub async fn send(&self, payload: &Payload) -> Result<(), NtfyError> {
        // Build request
        let mut builder = self.client.post(self.url.as_str());

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
            return Err(NtfyError::EmptyResponse);
        }

        // TODO: check the text?

        Ok(())
    }

    #[cfg(feature = "blocking")]
    /// Send payload to ntfy server
    pub fn send(&self, payload: &Payload) -> Result<(), NtfyError> {
        // Build request
        let mut builder = self.agent.post(self.url.as_str());

        // If markdown, set headers
        if payload.markdown {
            builder = builder.set("Markdown", "yes");
        }

        // Send request
        let res: Response = builder.send_json(payload)?;

        // Get full response text
        let text: String = res.into_string()?;

        if text.is_empty() {
            return Err(NtfyError::EmptyResponse);
        }

        // TODO: check the text?

        Ok(())
    }
}
