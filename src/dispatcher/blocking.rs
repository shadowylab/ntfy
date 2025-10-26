// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use ureq::config::ConfigBuilder;
use ureq::http::{HeaderValue, Request, Response};
use ureq::middleware::{Middleware, MiddlewareNext};
use ureq::typestate::AgentScope;
#[cfg(all(feature = "socks", not(target_arch = "wasm32")))]
use ureq::Proxy;
use ureq::{Agent, Body, SendBody};
use url::Url;

use super::builder::DispatcherBuilder;
use crate::error::Error;
use crate::payload::Payload;
use crate::Auth;

struct AuthMiddleware {
    header: HeaderValue,
}

impl AuthMiddleware {
    fn new(auth: Auth) -> Result<Self, Error> {
        Ok(Self {
            header: HeaderValue::from_str(&auth.to_header_value())?,
        })
    }
}

impl Middleware for AuthMiddleware {
    fn handle(
        &self,
        mut req: Request<SendBody>,
        next: MiddlewareNext,
    ) -> Result<Response<Body>, ureq::Error> {
        req.headers_mut()
            .insert("Authorization", self.header.clone());
        next.handle(req)
    }
}

/// Blocking dispatcher
#[derive(Debug, Clone)]
pub struct Blocking {
    client: Agent,
}

impl Blocking {
    #[inline]
    pub(crate) fn new(builder: DispatcherBuilder) -> Result<Self, Error> {
        Self::new_with_client(builder, Agent::config_builder())
    }

    pub(crate) fn new_with_client(
        builder: DispatcherBuilder,
        mut client: ConfigBuilder<AgentScope>,
    ) -> Result<Self, Error> {
        if let Some(auth) = builder.auth {
            // Construct middleware
            let middleware = AuthMiddleware::new(auth)?;

            // Set the authorization headers of every request using a middleware function
            client = client.middleware(middleware);
        }

        #[cfg(all(feature = "socks", not(target_arch = "wasm32")))]
        if let Some(proxy) = builder.proxy {
            let proxy = Proxy::new(&proxy)?;
            client = client.proxy(Some(proxy));
        }

        Ok(Self {
            client: client.build().into(),
        })
    }

    pub(crate) fn send(&self, url: &Url, payload: &Payload) -> Result<(), Error> {
        // Build request
        let builder = self.client.post(url.as_str());

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
