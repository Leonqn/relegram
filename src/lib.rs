extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate futures;
extern crate tokio;

pub use self::api::{HttpClient, BotApiClient};

pub mod requests;
pub mod error;
pub mod responses;

mod api;

pub(crate) mod try_from;
pub(crate) mod stream;