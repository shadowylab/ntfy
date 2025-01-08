// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let p: u8 = *self as u8;
        p.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            5 => Ok(Priority::Max),
            4 => Ok(Priority::High),
            3 => Ok(Priority::Default),
            2 => Ok(Priority::Low),
            1 => Ok(Priority::Min),
            o => Err(Error::custom(format_args!("Invalid value: {}", o))),
        }
    }
}
