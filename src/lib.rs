
extern crate hyper;
extern crate hyper_native_tls;
extern crate crypto;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod scrobbler;
mod client;
mod auth;
mod dto;

pub use scrobbler::Scrobbler;
