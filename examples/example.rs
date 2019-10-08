use rustfm_scrobble::{Scrobble, Scrobbler};

// Example rustfm-scrobble client showing authentication, now playing and
// scrobbling.
// Replace credential values with your own to test.

fn main() {
    let api_key = "{{api_key}}";
    let api_secret = "{{api_secret}}";
    let username = "{{username}}";
    let password = "{{password}}";

    let mut scrobbler = Scrobbler::new(api_key, api_secret);

    match scrobbler.authenticate_with_password(username, password) {
        Ok(_) => {
            println!("Authenticated!");
        }
        Err(e) => {
            println!("Authentication failed: {}", e);
        }
    };

    let track = Scrobble::new("Los Campesinos!", "As Lucerne / The Low", "No Blues");
    match scrobbler.now_playing(track.clone()) {
        Ok(resp) => {
            println!("Now playing: {} - {}", resp.artist.text, resp.track.text);
        }
        Err(e) => {
            println!("Now playing failed: {}", e);
        }
    }

    match scrobbler.scrobble(track) {
        Ok(resp) => {
            println!("Scrobbled {} - {} at timestamp: {}", resp.artist.text, resp.track.text, resp.timestamp);
        }
        Err(e) => {
            println!("Scrobble failed: {}", e);
        }
    }

}
