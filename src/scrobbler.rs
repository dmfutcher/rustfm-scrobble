use crate::client::LastFmClient;
use crate::models::metadata::{Scrobble, ScrobbleBatch};
use crate::models::responses::{
    BatchScrobbleResponse, NowPlayingResponse, ScrobbleResponse, SessionResponse,
};

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::result;
use std::time::{SystemTimeError, UNIX_EPOCH};

type Result<T> = result::Result<T, ScrobblerError>;

/// Submits song-play tracking information to Last.fm
pub struct Scrobbler {
    client: LastFmClient,
}

impl Scrobbler {
    /// Creates a new Scrobbler with the given Last.fm API Key and API Secret
    pub fn new(api_key: &str, api_secret: &str) -> Scrobbler {
        let client = LastFmClient::new(api_key, api_secret);

        Scrobbler { client }
    }

    pub fn authenticate_with_password(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<SessionResponse> {
        self.client.set_user_credentials(username, password);
        Ok(self.client.authenticate_with_password()?)
    }

    pub fn authenticate_with_token(&mut self, token: &str) -> Result<SessionResponse> {
        self.client.set_user_token(token);
        Ok(self.client.authenticate_with_token()?)
    }

    pub fn authenticate_with_session_key(&mut self, session_key: &str) {
        self.client.authenticate_with_session_key(session_key)
    }

    /// Registers the given track by the given artist as the currently authenticated user's
    /// "now playing" track.
    pub fn now_playing(&self, scrobble: Scrobble) -> Result<NowPlayingResponse> {
        let params = scrobble.as_map();

        Ok(self.client.send_now_playing(&params)?)
    }

    /// Registers a scrobble (play) of the track with the given title by the given artist in
    /// the account of the currently authenticated user at the current time.
    pub fn scrobble(&self, scrobble: Scrobble) -> Result<ScrobbleResponse> {
        let mut params = scrobble.as_map();
        let current_time = UNIX_EPOCH.elapsed()?;

        params
            .entry("timestamp".to_string())
            .or_insert_with(|| format!("{}", current_time.as_secs()));

        Ok(self.client.send_scrobble(&params)?)
    }

    pub fn scrobble_batch(&self, batch: ScrobbleBatch) -> Result<BatchScrobbleResponse> {
        let mut params = HashMap::new();

        let batch_count = batch.len();
        if batch_count > 50 {
            return Err(ScrobblerError::new(
                "Scrobble batch too large (must be 50 or fewer scrobbles)".to_owned(),
            ));
        } else if batch_count == 0 {
            return Err(ScrobblerError::new("Scrobble batch is empty".to_owned()));
        }

        for (i, scrobble) in batch.iter().enumerate() {
            let mut scrobble_params = scrobble.as_map();
            let current_time = UNIX_EPOCH.elapsed()?;
            scrobble_params
                .entry("timestamp".to_string())
                .or_insert_with(|| format!("{}", current_time.as_secs()));

            for (key, val) in scrobble_params.iter() {
                // batched parameters need array notation suffix ie.
                // "artist[1]"" = "Artist 1", "artist[2]" = "Artist 2"
                params.insert(format!("{}[{}]", key, i), val.clone());
            }
        }

        Ok(self.client.send_batch_scrobbles(&params)?)
    }

    /// Gets the session key the client is currently authenticated with. Returns
    /// `None` if not authenticated. Valid session keys can be stored and used
    /// to authenticate with `authenticate_with_session_key`.
    pub fn session_key(&self) -> Option<&str> {
        self.client.session_key()
    }
}

#[derive(Debug)]
pub struct ScrobblerError {
    err_msg: String,
}

impl ScrobblerError {
    pub fn new(err_msg: String) -> ScrobblerError {
        ScrobblerError { err_msg }
    }
}

impl fmt::Display for ScrobblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl Error for ScrobblerError {
    fn description(&self) -> &str {
        self.err_msg.as_str()
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl From<SystemTimeError> for ScrobblerError {
    fn from(error: SystemTimeError) -> Self {
        ScrobblerError::new(error.to_string())
    }
}

impl From<String> for ScrobblerError {
    fn from(error: String) -> Self {
        ScrobblerError::new(error)
    }
}
