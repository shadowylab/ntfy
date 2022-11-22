// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NtfyError {
    #[error("Failed to deserialize: {0}")]
    FailedToDeserialize(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(reqwest::Error),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(InvalidHeaderValue),
    #[error("Invalid url: {0}")]
    Url(url::ParseError),
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

impl From<url::ParseError> for NtfyError {
    fn from(err: url::ParseError) -> Self {
        NtfyError::Url(err)
    }
}
