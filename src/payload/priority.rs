// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Priority {
    Max = 5,
    High = 4,
    Default = 3,
    Low = 2,
    Min = 1,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Default
    }
}

pub(super) fn serialize<S>(p: &Priority, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let p: u8 = *p as u8;
    p.serialize(s)
}

pub(super) fn deserialize<'de, D>(d: D) -> Result<Priority, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(d)? {
        5 => Ok(Priority::Max),
        4 => Ok(Priority::High),
        3 => Ok(Priority::Default),
        2 => Ok(Priority::Low),
        1 => Ok(Priority::Min),
        o => Err(Error::custom(format_args!("Invalid value: {}", o))),
    }
}
