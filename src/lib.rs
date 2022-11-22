// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#![doc = include_str!("../README.md")]

#[macro_use]
extern crate serde;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
#[cfg(not(feature = "blocking"))]
use reqwest::Client as ReqwestClient;
#[cfg(not(feature = "blocking"))]
use reqwest::RequestBuilder;

#[cfg(feature = "blocking")]
use reqwest::blocking::Client as ReqwestClient;
#[cfg(feature = "blocking")]
use reqwest::blocking::RequestBuilder;
use reqwest::{Proxy, StatusCode};
use thiserror::Error;

mod auth;
mod payload;
mod priority;

pub use self::auth::Auth;
pub use self::payload::Payload;
pub use self::priority::Priority;

#[derive(Clone)]
pub struct Dispatcher {
    url: String,
    client: ReqwestClient,
}

#[derive(Debug, Error)]
pub enum NtfyError {
    #[error("Failed to deserialize: {0}")]
    FailedToDeserialize(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(reqwest::Error),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(InvalidHeaderValue),
    #[error("Empty Response")]
    EmptyResponse,
    #[error("Bad Result")]
    BadResult,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad Request")]
    BadRequest,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("Too Many Requests")]
    TooManyRequests,
    #[error("Unhandled Client Error")]
    UnhandledClientError,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Internal Server Error")]
    NotImplemented,
    #[error("Bad Gateway")]
    BadGateway,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("Gateway Timeout")]
    GatewayTimeout,
    #[error("Unhandled Server Error")]
    UnhandledServerError,
}

impl Dispatcher {
    /// Create new dispatcher
    pub fn new(url: &str, auth: Option<Auth>, proxy: Option<&str>) -> Result<Self, NtfyError> {
        let mut client = ReqwestClient::builder();

        if let Some(auth) = auth {
            let mut headers = HeaderMap::new();
            let mut auth_value = HeaderValue::from_str(&format!("Basic {}", auth.as_base64()))?;
            auth_value.set_sensitive(true);
            headers.insert("Authorization", auth_value);

            client = client.default_headers(headers);
        }

        if let Some(proxy) = proxy {
            client = client.proxy(Proxy::all(proxy)?);
        }

        Ok(Self {
            url: url.into(),
            client: client.build()?,
        })
    }

    /// Send payload to ntfy server
    #[cfg(not(feature = "blocking"))]
    pub async fn send(&self, payload: &Payload) -> Result<(), NtfyError> {
        log::debug!("{:?}", payload);
        request(self.client.post(&self.url).json(payload)).await
    }

    /// Send payload to ntfy server
    #[cfg(feature = "blocking")]
    pub fn send(&self, payload: &Payload) -> Result<(), NtfyError> {
        log::debug!("{:?}", payload);
        request(self.client.post(&self.url).json(payload))
    }
}

#[cfg(not(feature = "blocking"))]
async fn request(req: RequestBuilder) -> Result<(), NtfyError> {
    let res = req.send().await?;

    match StatusCode::as_u16(&res.status()) {
        0_u16..=399_u16 => {
            let res = res.text().await?;

            if res.is_empty() {
                return Err(NtfyError::EmptyResponse);
            }

            Ok(())
        }
        400 => Err(NtfyError::BadRequest),
        401 => Err(NtfyError::Unauthorized),
        402 => Err(NtfyError::UnhandledClientError),
        403 => Err(NtfyError::Forbidden),
        404 => Err(NtfyError::NotFound),
        405 => Err(NtfyError::MethodNotAllowed),
        406_u16..=428_u16 => Err(NtfyError::UnhandledClientError),
        429 => Err(NtfyError::TooManyRequests),
        430_u16..=499_u16 => Err(NtfyError::UnhandledClientError),
        500 => Err(NtfyError::InternalServerError),
        501 => Err(NtfyError::NotImplemented),
        502 => Err(NtfyError::BadGateway),
        503 => Err(NtfyError::ServiceUnavailable),
        504 => Err(NtfyError::GatewayTimeout),
        _ => Err(NtfyError::UnhandledServerError),
    }
}

#[cfg(feature = "blocking")]
fn request(req: RequestBuilder) -> Result<(), Error> {
    let res = req.send()?;

    match StatusCode::as_u16(&res.status()) {
        0_u16..=399_u16 => {
            let res = res.text()?;

            if res.is_empty() {
                return Err(NtfyError::EmptyResponse);
            }

            Ok(())
        }
        400 => Err(NtfyError::BadRequest),
        401 => Err(NtfyError::Unauthorized),
        402 => Err(NtfyError::UnhandledClientError),
        403 => Err(NtfyError::Forbidden),
        404 => Err(NtfyError::NotFound),
        405 => Err(NtfyError::MethodNotAllowed),
        406_u16..=428_u16 => Err(NtfyError::UnhandledClientError),
        429 => Err(NtfyError::TooManyRequests),
        430_u16..=499_u16 => Err(NtfyError::UnhandledClientError),
        500 => Err(NtfyError::InternalServerError),
        501 => Err(NtfyError::NotImplemented),
        502 => Err(NtfyError::BadGateway),
        503 => Err(NtfyError::ServiceUnavailable),
        504 => Err(NtfyError::GatewayTimeout),
        _ => Err(NtfyError::UnhandledServerError),
    }
}

impl From<reqwest::Error> for NtfyError {
    fn from(err: reqwest::Error) -> Self {
        NtfyError::ReqwestError(err)
    }
}

impl From<InvalidHeaderValue> for NtfyError {
    fn from(err: InvalidHeaderValue) -> Self {
        NtfyError::InvalidHeaderValue(err)
    }
}
