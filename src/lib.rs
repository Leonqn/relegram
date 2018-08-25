extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate futures;
extern crate tokio;

pub mod requests;
pub mod error;
pub mod api;
pub mod responses;
pub mod stream;


pub(crate) trait TryFrom<T>: Sized {
    type Error;

    fn try_from(value: T) -> Result<Self, Self::Error>;
}