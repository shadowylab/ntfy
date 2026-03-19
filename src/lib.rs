// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

//! Ntfy

#![cfg_attr(feature = "async", doc = include_str!("../README.md"))]

#[cfg(not(any(
    feature = "async-dispatcher",
    feature = "async-subscriber",
    feature = "blocking-dispatcher",
    feature = "blocking-subscriber"
)))]
compile_error!(
    "at least one of the `async-dispatcher`, `async-subscriber`, `blocking-dispatcher`, or `blocking-subscriber` features must be enabled"
);

#[cfg(all(
    any(feature = "blocking-dispatcher", feature = "blocking-subscriber"),
    target_arch = "wasm32"
))]
compile_error!("`blocking` features can't be enabled on WASM");

#[macro_use]
extern crate serde;

pub use url::Url;

pub mod auth;
#[cfg(any(feature = "async-dispatcher", feature = "blocking-dispatcher"))]
pub mod dispatcher;
pub mod error;
pub mod payload;
pub mod prelude;
#[cfg(any(feature = "async-subscriber", feature = "blocking-subscriber"))]
pub mod subscriber;

pub use self::auth::Auth;
#[cfg(any(feature = "async-dispatcher", feature = "blocking-dispatcher"))]
pub use self::dispatcher::{Dispatcher, DispatcherBuilder};
pub use self::error::Error;
pub use self::payload::{Payload, Priority};
#[cfg(any(feature = "async-subscriber", feature = "blocking-subscriber"))]
pub use self::subscriber::{Subscriber, SubscriberBuilder};
