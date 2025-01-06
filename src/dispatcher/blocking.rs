// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use ureq::{Agent, AgentBuilder, MiddlewareNext, Request, Response};
use url::Url;

use super::builder::DispatcherBuilder;
use crate::error::Error;
use crate::payload::Payload;

/// Blocking dispatcher
#[derive(Debug, Clone)]
pub struct Blocking {
    client: Agent,
}

impl Blocking {
    #[inline]
    pub(crate) fn new(builder: DispatcherBuilder) -> Result<Self, Error> {
        Self::new_with_client(builder, ureq::builder())
    }

    pub(crate) fn new_with_client(
        builder: DispatcherBuilder,
        mut client: AgentBuilder,
    ) -> Result<Self, Error> {
        if let Some(auth) = builder.auth {
            let heaver_value = auth.to_header_value();

            // Set the authorization headers of every request using a middleware function
            client = client.middleware(
                move |req: Request, next: MiddlewareNext| -> Result<Response, ureq::Error> {
                    next.handle(req.set("Authorization", &heaver_value))
                },
            );
        }

        if let Some(proxy) = builder.proxy {
            let proxy = ureq::Proxy::new(proxy)?;
            client = client.proxy(proxy);
        }

        Ok(Self {
            client: client.build(),
        })
    }

    pub(crate) fn send(&self, url: &Url, payload: &Payload) -> Result<(), Error> {
        // Build request
        let mut builder = self.client.post(url.as_str());

        // If markdown, set headers
        if payload.markdown {
            builder = builder.set("Markdown", "yes");
        }

        // Send request
        let res: Response = builder.send_json(payload)?;

        // Get full response text
        let text: String = res.into_string()?;

        if text.is_empty() {
            return Err(Error::EmptyResponse);
        }

        // TODO: check the text?

        Ok(())
    }
}
