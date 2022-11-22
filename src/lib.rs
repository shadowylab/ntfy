// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#![doc = include_str!("../README.md")]

#[macro_use]
extern crate serde;

pub mod dispatcher;
pub mod error;
mod net;
pub mod payload;

pub use self::dispatcher::{Auth, Dispatcher, DispatcherBuilder};
pub use self::error::NtfyError;
pub use self::payload::{Payload, Priority};
