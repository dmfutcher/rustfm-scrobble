//! # rustfm-scrobble
//!
//! Client for the Last.fm Scrobble API v2.0.

#[macro_use]
extern crate wrapped_vec;

mod scrobbler;
mod client;
mod auth;
mod models;

pub use crate::scrobbler::{Scrobbler, ScrobblerError};
pub use crate::models::metadata::{Scrobble, ScrobbleBatch};

pub mod responses {
    pub use crate::models::responses::{SessionResponse, NowPlayingResponse, ScrobbleResponse,
                                BatchScrobbleResponse};

    pub mod values {
        pub use crate::models::responses::{CorrectableString, ScrobbleList};
    }
}
