use client::LastFmClient;

use std::collections::HashMap;
use std::time::UNIX_EPOCH;

/// Submits song-play tracking information to Last.fm
pub struct Scrobbler {
    client: LastFmClient
}

impl Scrobbler {

    /// Creates a new Scrobbler with the given Last.fm API Key and API Secret
    pub fn new(api_key: String, api_secret: String) -> Scrobbler {
        let client = LastFmClient::new(api_key, api_secret);

        Scrobbler{
            client: client
        }
    }

    /// Uses the given username and password (for the user to log scrobbles against), plus
    /// the API key and API secret to authenticate with Last.fm API using 'getMobileSession'
    /// authentication scheme.
    pub fn authenticate(&mut self, username: String, password: String) -> Result<(), String> {
        self.client.set_user_credentials(username, password);
        self.client.authenticate()
    }

    /// Registers the given track by the given artist as the currently authenticated user's
    /// "now playing" track.
    pub fn now_playing(&self, name: String, artist: String) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("track", name);
        params.insert("artist", artist);

        match self.client.send_authenticated_request("track.updateNowPlaying", &params) {
            Ok(_) => Ok(()),
            Err(msg) => Err(msg)
        }
    }

    /// Registers a scrobble (play) of the track with the given title by the given artist in
    /// the account of the currently authenticated user at the current time.
    pub fn scrobble(&self, name: String, artist: String) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("track", name);
        params.insert("artist", artist);
        params.insert("timestamp", format!("{}", UNIX_EPOCH.elapsed().unwrap().as_secs()));

        match self.client.send_authenticated_request("track.scrobble", &params) {
            Ok(body) => {
                println!("Body: {}", body);
                Ok(())
            },
            Err(msg) => Err(msg)
        }
    }

}
