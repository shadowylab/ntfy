// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#[macro_use]
extern crate serde;

use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::RequestBuilder;
use reqwest::{Proxy, StatusCode};

mod priority;

pub use priority::Priority;

#[derive(Clone)]
pub struct Dispatcher {
    url: String,
    client: ReqwestClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub topic: String,
    pub message: String,
    #[serde(with = "priority")]
    pub priority: Priority,
    pub title: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    FailedToDeserialize(String),
    ReqwestError(reqwest::Error),
    EmptyResponse,
    BadResult,
    Unauthorized,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    TooManyRequests,
    UnhandledClientError,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    UnhandledServerError,
}

impl Dispatcher {
    pub fn new(url: &str, proxy: Option<&str>) -> Result<Self, Error> {
        let mut client = ReqwestClient::builder();

        if let Some(proxy) = proxy {
            client = client.proxy(Proxy::all(proxy)?);
        }

        Ok(Self {
            url: url.into(),
            client: client.build()?,
        })
    }

    pub fn send(&self, payload: &Payload) -> Result<(), Error> {
        log::debug!("{:?}", payload);
        request(self.client.post(&self.url).json(payload))
    }
}

fn request(req: RequestBuilder) -> Result<(), Error> {
    let res = req.send()?;

    match StatusCode::as_u16(&res.status()) {
        0_u16..=399_u16 => {
            let res = res.text()?;

            if res.is_empty() {
                return Err(Error::EmptyResponse);
            }

            Ok(())
        }
        400 => Err(Error::BadRequest),
        401 => Err(Error::Unauthorized),
        402 => Err(Error::UnhandledClientError),
        403 => Err(Error::Forbidden),
        404 => Err(Error::NotFound),
        405 => Err(Error::MethodNotAllowed),
        406_u16..=428_u16 => Err(Error::UnhandledClientError),
        429 => Err(Error::TooManyRequests),
        430_u16..=499_u16 => Err(Error::UnhandledClientError),
        500 => Err(Error::InternalServerError),
        501 => Err(Error::NotImplemented),
        502 => Err(Error::BadGateway),
        503 => Err(Error::ServiceUnavailable),
        504 => Err(Error::GatewayTimeout),
        _ => Err(Error::UnhandledServerError),
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}
