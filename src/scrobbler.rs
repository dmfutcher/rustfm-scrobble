use client::LastFmClient;

use std::collections::HashMap;

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

    pub fn authenticate(&mut self, username: String, password: String) -> Result<(), String> {
        self.client.set_user_credentials(username, password);
        self.client.authenticate()
    }

    pub fn now_playing(&self, name: String, artist: String) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("track", name);
        params.insert("artist", artist);

        match self.client.send_authenticated_request("track.updateNowPlaying", &params) {
            Ok(_) => Ok(()),
            Err(msg) => Err(msg)
        }
    }

    pub fn send_scrobble() -> Result<(), &'static str> {
        Err("Not implemented")
    }

}
