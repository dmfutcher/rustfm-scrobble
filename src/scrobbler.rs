use client::{LastFmClient};

pub struct Scrobbler {
    client: LastFmClient
}

impl Scrobbler {

    pub fn new(api_key: String, api_secret: String) -> Scrobbler {
        let client = LastFmClient::new(api_key, api_secret);

        Scrobbler{
            client: client
        }
    }

    pub fn authenticate(username: String, password: String) -> Result<(), &'static str> {
        Err("Not implemented")
    }

    pub fn send_now_playing() -> Result<(), &'static str> {
        Err("Not implemented")
    }

    pub fn send_scrobble() -> Result<(), &'static str> {
        Err("Not implemented")
    }

}
