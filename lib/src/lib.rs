//! This crate helps writing bots for the messenger Telegram.
//! See [readme](https://github.com/telegram-rs/telegram-bot) for details.
#![feature(box_syntax, try_trait, generators)]

extern crate antidote;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate http;
extern crate telegram_bot_async_raw;
extern crate tokio;

#[cfg(feature = "hyper_connector")]
extern crate hyper;
#[cfg(feature = "hyper_connector")]
extern crate hyper_tls;

mod api;
mod errors;
mod macros;
mod stream;

pub mod connector;
pub mod prelude;
pub mod types;

pub use self::{
    api::{Api, DefaultApi},
    connector::*,
    errors::Error,
    prelude::*,
    stream::UpdatesStream,
    types::*,
};
