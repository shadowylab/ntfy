// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::fmt;

use base64::engine::{general_purpose, Engine};

#[derive(Clone)]
pub struct Auth {
    username: String,
    password: String,
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Auth(<sensitive>)")
    }
}

impl Auth {
    pub fn new<S>(username: S, password: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    #[inline]
    pub(crate) fn as_base64(&self) -> String {
        general_purpose::STANDARD.encode(format!("{}:{}", self.username, self.password))
    }
}
