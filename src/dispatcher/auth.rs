// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#[derive(Clone)]
pub struct Auth {
    username: String,
    password: String,
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

    pub fn as_base64(&self) -> String {
        base64::encode(format!("{}:{}", self.username, self.password))
    }
}
