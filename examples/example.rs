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

    let track = Scrobble::new("Los Campesinos!", "As Lucerne / The Low", "No Blues");
    let response = scrobbler.now_playing(track.clone())?;
    println!("Sent now playing! {:#?}", response);

    let response = scrobbler.scrobble(track)?;
    println!("Sent scrobble! {:#?}", response);

    Ok(())
}
