// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::fmt;

use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum NtfyError {
    ReqwestError(reqwest::Error),
    Url(url::ParseError),
    InvalidHeaderValue(InvalidHeaderValue),
    EmptyResponse,
}

impl std::error::Error for NtfyError {}

impl fmt::Display for NtfyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReqwestError(e) => write!(f, "{}", e),
            Self::Url(e) => write!(f, "{}", e),
            Self::InvalidHeaderValue(e) => write!(f, "{}", e),
            Self::EmptyResponse => write!(f, "Empty Response"),
        }
    }
}

impl From<reqwest::Error> for NtfyError {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<url::ParseError> for NtfyError {
    fn from(e: url::ParseError) -> Self {
        Self::Url(e)
    }
}

impl From<InvalidHeaderValue> for NtfyError {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(e)
    }
}
