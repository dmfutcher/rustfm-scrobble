use client::LastFmClient;
use models::responses::{SessionResponse, NowPlayingResponse, ScrobbleResponse};

use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use std::error::Error;
use std::fmt;
use std::result;

type Result<T> = result::Result<T, ScrobblerError>;

/// Submits song-play tracking information to Last.fm
pub struct Scrobbler {
    client: LastFmClient,
}

impl Scrobbler {
    /// Creates a new Scrobbler with the given Last.fm API Key and API Secret
    pub fn new(api_key: String, api_secret: String) -> Scrobbler {
        let client = LastFmClient::new(api_key, api_secret);

        Scrobbler { client: client }
    }

    /// Uses the given username and password (for the user to log scrobbles against), plus
    /// the API key and API secret to authenticate with Last.fm API using 'getMobileSession'
    /// authentication scheme.
    pub fn authenticate(&mut self, username: String, password: String) -> Result<SessionResponse> {
        self.client.set_user_credentials(username, password);
        self.client
            .authenticate()
            .map_err(ScrobblerError::new)
    }

    /// Registers the given track by the given artist as the currently authenticated user's
    /// "now playing" track.
    pub fn now_playing(&self, name: String, artist: String) -> Result<NowPlayingResponse> {
        let mut params = HashMap::new();
        params.insert("track", name);
        params.insert("artist", artist);

        self.client
            .send_now_playing(&params)
            .map_err(ScrobblerError::new)
    }

    /// Registers a scrobble (play) of the track with the given title by the given artist in
    /// the account of the currently authenticated user at the current time.
    pub fn scrobble(&self, name: String, artist: String) -> Result<ScrobbleResponse> {
        let mut params = HashMap::new();
        params.insert("track", name);
        params.insert("artist", artist);
        params.insert("timestamp",
                      format!("{}", UNIX_EPOCH.elapsed().unwrap().as_secs()));

        self.client
            .send_scrobble(&params)
            .map_err(ScrobblerError::new)
    }
}

#[derive(Debug)]
pub struct ScrobblerError {
    err_msg: String,
}

impl ScrobblerError {
    pub fn new(err_msg: String) -> ScrobblerError {
        ScrobblerError { err_msg: err_msg }
    }
}

impl fmt::Display for ScrobblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl Error for ScrobblerError {
    fn description(&self) -> &str {
        self.err_msg.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
