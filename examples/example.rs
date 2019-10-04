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
            println!("{}", e);
        }
    };

    let track_one = Scrobble::new("Los Campesinos!", "As Lucerne / The Low", "No Blues");
    match scrobbler.now_playing(track_one) {
        Ok(_) => {
            println!("Sent now playing! ");
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let track_two = Scrobble::new("Los Campesinos!", "The Time Before the Last", "No Blues");
    match scrobbler.scrobble(track_two) {
        Ok(_) => {
            println!("Sent scrobble!");
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let track_three = Scrobble::new("Los Campesinos!", "Selling Rope", "No Blues");
    match scrobbler.now_playing(track_three) {
        Ok(_) => {
            println!("Sent now playing! ");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
