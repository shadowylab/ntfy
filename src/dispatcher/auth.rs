// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::fmt;

use base64::engine::{general_purpose, Engine};

#[derive(Clone)]
pub enum Auth {
    /// Authentication with credential
    Credentials {
        /// Username
        username: String,
        /// Password
        password: String,
    },
    /// Authentication with access token
    Token(String),
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Auth(<sensitive>)")
    }
}

impl Auth {
    #[deprecated(since = "0.6.0", note = "Use `Auth::credentials` instead.")]
    pub fn new<S>(username: S, password: S) -> Self
    where
        S: Into<String>,
    {
        Self::credentials(username, password)
    }

    /// Authentication with credential (username + password)
    ///
    /// <https://docs.ntfy.sh/publish/#username-password>
    #[inline]
    pub fn credentials<U, P>(username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        Self::Credentials {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Authentication with access token
    ///
    /// <https://docs.ntfy.sh/publish/#access-tokens>
    #[inline]
    pub fn token<S>(token: S) -> Self
    where
        S: Into<String>,
    {
        Self::Token(token.into())
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_header_value(self) -> String {
        match self {
            Self::Credentials { username, password } => {
                let b64: String =
                    general_purpose::STANDARD.encode(format!("{username}:{password}"));
                format!("Basic {b64}")
            }
            Self::Token(token) => {
                format!("Bearer {token}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_credentials() {
        let auth = Auth::credentials("username", "password");
        assert_eq!(auth.to_header_value(), "Basic dXNlcm5hbWU6cGFzc3dvcmQ=");
    }

    #[test]
    fn test_auth_token() {
        let auth = Auth::token("tk_AgQdq7mVBoFD37zQVN29RhuMzNIz2");
        assert_eq!(
            auth.to_header_value(),
            "Bearer tk_AgQdq7mVBoFD37zQVN29RhuMzNIz2"
        );
    }
}
