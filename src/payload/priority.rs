// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::fmt;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Max = 5,
    High = 4,
    #[default]
    Default = 3,
    Low = 2,
    Min = 1,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Priority {
    pub fn from_u8(priority: u8) -> Result<Self, Error> {
        match priority {
            5 => Ok(Priority::Max),
            4 => Ok(Priority::High),
            3 => Ok(Priority::Default),
            2 => Ok(Priority::Low),
            1 => Ok(Priority::Min),
            _ => Err(Error::UnknownPriority),
        }
    }

    /// Convert to `u8`
    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Convert to `&str`
    pub fn as_str(&self) -> &str {
        match self {
            Self::Max => "max",
            Self::High => "high",
            Self::Default => "default",
            Self::Low => "low",
            Self::Min => "min",
        }
    }
}

impl FromStr for Priority {
    type Err = Error;

    fn from_str(priority: &str) -> Result<Self, Self::Err> {
        match priority {
            "max" | "urgent" => Ok(Self::Max),
            "high" => Ok(Self::High),
            "default" => Ok(Self::Default),
            "low" => Ok(Self::Low),
            "min" => Ok(Self::Min),
            _ => Err(Error::UnknownPriority),
        }
    }
}

impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // According to ntfy docs, the priority in the JSON payload must be a number.
        // https://docs.ntfy.sh/subscribe/api/#json-message-format
        serializer.serialize_u8(self.as_u8())
    }
}

impl<'de> Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let priority: u8 = u8::deserialize(deserializer)?;
        Priority::from_u8(priority).map_err(de::Error::custom)
    }
}
