//! # rustfm-scrobble
//!
//! Client for the Last.fm Scrobble API v2.0.

extern crate reqwest;
extern crate crypto;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod scrobbler;
mod client;
mod auth;
mod models;

pub use scrobbler::{Scrobbler, ScrobblerError};

pub mod responses {
    pub use models::responses::{SessionResponse, NowPlayingResponse, ScrobbleResponse,
                                CorrectableString};
}

pub mod metadata {
    // TODO: Should this be exposed in the root?
    pub use models::metadata::Scrobble;
}
