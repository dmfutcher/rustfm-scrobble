#![deny(clippy::all)]
#![deny(clippy::pedantic)]
//! # rustfm-scrobble
//!
//! Client for the Last.fm Scrobble API v2.0.

#[macro_use]
extern crate wrapped_vec;

mod auth;
mod client;
mod error;
mod models;
mod scrobbler;

pub use crate::models::metadata::{Scrobble, ScrobbleBatch};
pub use crate::scrobbler::Scrobbler;
pub use crate::error::Error;

pub mod responses {
    pub use crate::models::responses::{
        BatchScrobbleResponse, NowPlayingResponse, ScrobbleResponse, SessionResponse,
    };

    pub mod values {
        pub use crate::models::responses::{CorrectableString, ScrobbleList};
    }
}
