extern crate rustfm_scrobble;

use rustfm_scrobble::{Scrobbler, Scrobble};

// Example rustfm-scrobble client showing authentication, now playing and
// scrobbling.
// Replace credential values with your own to test.

fn main() {
    let api_key = "{{api_key}}".to_string();
    let api_secret = "{{api_secret}}".to_string();
    let username = "{{username}}".to_string();
    let password = "{{password}}".to_string();

    let mut scrobbler = Scrobbler::new(api_key, api_secret);

    match scrobbler.authenticate(username, password) {
        Ok(_) => { println!("Authenticated!"); }
        Err(e) => { println!("{}", e); }
    };

    let artist = "Los Campesinos!".to_string();
    let album = "No Blues".to_string();

    let track_one = Scrobble::new("As Lucerne / The Low".to_string(), artist.clone(), album.clone());
    match scrobbler.now_playing(track_one) {
        Ok(_) => { println!("Sent now playing! "); }
        Err(e) => { println!("{}", e); }
    }

    let track_two = Scrobble::new("The Time Before the Last".to_string(), artist.clone(), album.clone());
    match scrobbler.scrobble(track_two) {
        Ok(_) => { println!("Sent scrobble!"); }
        Err(e) => { println!("{}", e); }
    }

    let track_three = Scrobble::new("Selling Rope".to_string(), artist.clone(), album.clone());
    match scrobbler.now_playing(track_three) {
        Ok(_) => { println!("Sent now playing! "); }
        Err(e) => { println!("{}", e); }
    }
}
