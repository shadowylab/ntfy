// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NtfyError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Empty Response")]
    EmptyResponse,
}
