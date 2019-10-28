#![deny(clippy::all)]
#![deny(clippy::pedantic)]
//! # rustfm-scrobble
//!
//! Client for the Last.fm Scrobble API v2.0. Allows easy access to the most-commonly used Scrobble/Now Playing
//! endpoints in the Last.fm API, as well as robust support for multiple authentication flows. More advanced API
//! features such as metadata correction are also exposed to help build more sophisticated Scrobble clients. 
//! 
//! The primary types to use are `Scrobbler` - the actual client, which you will authenticate and then use to send 
//! scrobble requests - and `Scrobble` - which represents a single track played at a point in time. An example using
//! these types to scrobble a track to Last.fm is given below.
//! 
//! # Example usage
//! ```
//! use rustfm_scrobble::{Scrobble, Scrobbler};
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!    let api_key = "{{api_key}}";
//!    let api_secret = "{{api_secret}}";
//!    let username = "{{username}}";
//!    let password = "{{password}}";
//!
//!    let mut scrobbler = Scrobbler::new(api_key, api_secret);
//!
//!    let response = scrobbler.authenticate_with_password(username, password)?;
//!    println!("Authenticated! {:#?}", response);
//!
//!    let track = Scrobble::new("Los Campesinos!", "To Tundra", "No Blues");
//!    let response = scrobbler.now_playing(&track)?;
//!    println!("Sent now playing! {:#?}", response);
//!
//!    let response = scrobbler.scrobble(&track)?;
//!    println!("Sent scrobble! {:#?}", response);
//!
//!    Ok(())
//! }
//! ```
//! 
//! *Note:* This crate does not implement any of the logic to comply with Last.fm's scrobbling rules. Typical
//! ("real-time") implementations will likely want to adhere to these rules, outlined in Last.fm's 
//! [API Documentation](https://www.last.fm/api/scrobbling#scrobble-requests). Other implementations may choose to
//! ignore these guidelines. This crate provides the flexibility to develop any type of Scrobbling application.
//! 
#[macro_use]
extern crate wrapped_vec;

mod auth;
mod client;
mod error;
mod models;
mod scrobbler;

pub use crate::models::metadata::{Scrobble, ScrobbleBatch};
pub use crate::scrobbler::Scrobbler;
pub use crate::error::ScrobblerError;


/// Last.fm API Response Types
/// 
/// Types used to represent responses from the Last.fm API
pub mod responses {
    pub use crate::models::responses::{
        BatchScrobbleResponse, NowPlayingResponse, ScrobbleResponse, SessionResponse,
    };

    /// Data types used to represent values in API Response types
    pub mod values {
        pub use crate::models::responses::{CorrectableString, ScrobbleList};
    }
}
