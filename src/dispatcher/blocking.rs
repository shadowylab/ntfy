// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license
use url::Url;

use ureq::config::ConfigBuilder;
use ureq::http::{HeaderValue, Request, Response};
use ureq::middleware::MiddlewareNext;
use ureq::typestate::AgentScope;
use ureq::{Agent, Body, SendBody};

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
        Self::new_with_client(builder, ureq::Agent::config_builder())
    }

    pub(crate) fn new_with_client(
        builder: DispatcherBuilder,
        mut client: ConfigBuilder<AgentScope>,
    ) -> Result<Self, Error> {
        if let Some(auth) = builder.auth {
            let header_value = HeaderValue::from_str(&auth.to_header_value())?;

            // Set the authorization headers of every request using a middleware function
            client = client.middleware(
                move |mut req: Request<SendBody>,
                      next: MiddlewareNext|
                      -> Result<Response<Body>, ureq::Error> {
                    req.headers_mut()
                        .insert("Authorization", header_value.clone());
                    next.handle(req)
                },
            );
        }

        if let Some(proxy) = builder.proxy {
            let proxy = ureq::Proxy::new(&proxy)?;
            client = client.proxy(Some(proxy));
        }

        Ok(Self {
            client: client.build().into(),
        })
    }

    pub(crate) fn send(&self, url: &Url, payload: &Payload) -> Result<(), Error> {
        // Build request
        let mut builder = self.client.post(url.as_str());

        // If markdown, set headers
        if payload.markdown {
            builder = builder.header("Markdown", "yes");
        }

        // Send request
        let res: Response<Body> = builder.send_json(payload)?;

        // Get full response text
        let text = res.into_body().read_to_string()?;

        if text.is_empty() {
            return Err(Error::EmptyResponse);
        }

        // TODO: check the text?

        Ok(())
    }
}
