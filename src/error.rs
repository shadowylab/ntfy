// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::{fmt, io};

#[cfg(not(feature = "blocking"))]
use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum NtfyError {
    #[cfg(not(feature = "blocking"))]
    ReqwestError(reqwest::Error),
    #[cfg(feature = "blocking")]
    UreqError(ureq::Error),
    IoError(io::Error),
    Url(url::ParseError),
    #[cfg(not(feature = "blocking"))]
    InvalidHeaderValue(InvalidHeaderValue),
    EmptyResponse,
}

impl std::error::Error for NtfyError {}

impl fmt::Display for NtfyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(not(feature = "blocking"))]
            Self::ReqwestError(e) => write!(f, "{}", e),
            #[cfg(feature = "blocking")]
            Self::UreqError(e) => write!(f, "{}", e),
            Self::IoError(e) => write!(f, "{}", e),
            Self::Url(e) => write!(f, "{}", e),
            #[cfg(not(feature = "blocking"))]
            Self::InvalidHeaderValue(e) => write!(f, "{}", e),
            Self::EmptyResponse => write!(f, "Empty Response"),
        }
    }
}

#[cfg(not(feature = "blocking"))]
impl From<reqwest::Error> for NtfyError {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

#[cfg(feature = "blocking")]
impl From<ureq::Error> for NtfyError {
    fn from(e: ureq::Error) -> Self {
        Self::UreqError(e)
    }
}

impl From<io::Error> for NtfyError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<url::ParseError> for NtfyError {
    fn from(e: url::ParseError) -> Self {
        Self::Url(e)
    }
}

#[cfg(not(feature = "blocking"))]
impl From<InvalidHeaderValue> for NtfyError {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(e)
    }
}
