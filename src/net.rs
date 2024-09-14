// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use reqwest::RequestBuilder;
use reqwest::StatusCode;

use crate::error::NtfyError;

pub(crate) async fn request(req: RequestBuilder) -> Result<(), NtfyError> {
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
