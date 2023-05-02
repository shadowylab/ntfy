// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NtfyError {
    #[error("Failed to deserialize: {0}")]
    FailedToDeserialize(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Invalid url: {0}")]
    Url(#[from] url::ParseError),
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
