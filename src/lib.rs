// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

//! Ntfy

#![cfg_attr(feature = "async", doc = include_str!("../README.md"))]

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("at least one of the `async` or `blocking` features must be enabled");

#[macro_use]
extern crate serde;

pub use url::Url;

pub mod dispatcher;
pub mod error;
pub mod payload;
pub mod prelude;

pub use self::dispatcher::{Auth, Dispatcher, DispatcherBuilder};
pub use self::error::Error;
pub use self::payload::{Payload, Priority};
