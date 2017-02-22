extern crate scrobbler;

use scrobbler::Scrobbler;

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

    match scrobbler.now_playing("You! Me! Dancing!".to_string(), "Los Campesinos!".to_string()) {
        Ok(_) => { println!("Sent now playing! "); }
        Err(e) => { println!("{}", e); }
    }

    match scrobbler.scrobble("To Tundra".to_string(), "Los Campesinos!".to_string()) {
        Ok(_) => { println!("Sent scrobble!"); }
        Err(e) => { println!("{}", e); }
    }

    match scrobbler.now_playing("Sad Suppers".to_string(), "Los Campesinos!".to_string()) {
        Ok(_) => { println!("Sent now playing! "); }
        Err(e) => { println!("{}", e); }
    }
}
