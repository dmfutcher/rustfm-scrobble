//! # rustfm-scrobble
//!
//! Client for the Last.fm Scrobble API v2.0.

extern crate reqwest;
extern crate crypto;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate wrapped_vec;

mod scrobbler;
mod client;
mod auth;
mod models;

pub use scrobbler::{Scrobbler, ScrobblerError};
pub use models::metadata::{Scrobble, ScrobbleBatch};

pub mod responses {
    pub use models::responses::{SessionResponse, NowPlayingResponse, ScrobbleResponse,
                                BatchScrobbleResponse, CorrectableString};
}
