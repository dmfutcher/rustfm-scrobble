use rustfm_scrobble::{Scrobble, Scrobbler};
use std::error::Error;

// Example rustfm-scrobble client showing authentication, now playing and
// scrobbling.
// Replace credential values with your own to test.

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "{{api_key}}";
    let api_secret = "{{api_secret}}";
    let username = "{{username}}";
    let password = "{{password}}";

    let mut scrobbler = Scrobbler::new(api_key, api_secret);

    let response = scrobbler.authenticate_with_password(username, password)?;
    println!("Authenticated! {:#?}", response);

    let track_one = Scrobble::new("Los Campesinos!", "As Lucerne / The Low", "No Blues");
    let response = scrobbler.now_playing(track_one)?;
    println!("Sent now playing! {:#?}", response);

    let track_two = Scrobble::new("Los Campesinos!", "The Time Before the Last", "No Blues");
    let response = scrobbler.scrobble(track_two)?;
    println!("Sent scrobble! {:#?}", response);

    let track_three = Scrobble::new("Los Campesinos!", "Selling Rope", "No Blues");
    let response = scrobbler.now_playing(track_three)?;
    println!("Sent now playing! {:#?}", response);

    Ok(())
}
