// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::fmt;
#[cfg(feature = "blocking")]
use std::io;

use http::header::InvalidHeaderValue;

#[deprecated(since = "0.7.0", note = "Please use `Error` instead")]
pub type NtfyError = Error;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "async")]
    Reqwest(reqwest::Error),
    #[cfg(feature = "blocking")]
    Ureq(Box<ureq::Error>),
    #[cfg(feature = "blocking")]
    Io(io::Error),
    #[cfg(any(feature = "async-subscribing", feature = "blocking-subscribing"))]
    Tungstenite(tungstenite::Error),
    Serde(serde_json::Error),
    Url(url::ParseError),
    InvalidHeaderValue(InvalidHeaderValue),
    EmptyResponse,
    UnknownPriority,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "async")]
            Self::Reqwest(e) => write!(f, "{}", e),
            #[cfg(feature = "blocking")]
            Self::Ureq(e) => write!(f, "{}", e),
            #[cfg(feature = "blocking")]
            Self::Io(e) => write!(f, "{}", e),
            #[cfg(any(feature = "async-subscribing", feature = "blocking-subscribing"))]
            Self::Tungstenite(e) => write!(f, "{}", e),
            Self::Serde(e) => write!(f, "{}", e),
            Self::Url(e) => write!(f, "{}", e),
            Self::InvalidHeaderValue(e) => write!(f, "{}", e),
            Self::EmptyResponse => write!(f, "Empty response"),
            Self::UnknownPriority => write!(f, "Unknown priority"),
        }
    }
}

#[cfg(feature = "async")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

#[cfg(feature = "blocking")]
impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Self::Ureq(Box::new(e))
    }
}

#[cfg(feature = "blocking")]
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[cfg(any(feature = "async-subscribing", feature = "blocking-subscribing"))]
impl From<tungstenite::Error> for Error {
    fn from(e: tungstenite::Error) -> Self {
        Self::Tungstenite(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::Url(e)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(e)
    }
}
