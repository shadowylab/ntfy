// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

//! Ntfy

#![cfg_attr(feature = "async", doc = include_str!("../README.md"))]

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("at least one of the `async` or `blocking` features must be enabled");

#[cfg(all(feature = "blocking", target_arch = "wasm32"))]
compile_error!("`blocking` features can't be enabled on WASM");

#[macro_use]
extern crate serde;

pub use url::Url;

pub mod auth;
pub mod dispatcher;
pub mod error;
pub mod payload;
pub mod prelude;
#[cfg(feature = "blocking-subscribing")]
pub mod subscriber;

pub use self::auth::Auth;
pub use self::dispatcher::{Dispatcher, DispatcherBuilder};
pub use self::error::Error;
pub use self::payload::{Payload, Priority};
#[cfg(feature = "blocking-subscribing")]
pub use self::subscriber::{Subscriber, SubscriberBuilder};
