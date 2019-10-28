use crate::client::LastFm;
use crate::error::ScrobblerError;
use crate::models::metadata::{Scrobble, ScrobbleBatch};
use crate::models::responses::{
    BatchScrobbleResponse, NowPlayingResponse, ScrobbleResponse, SessionResponse,
};

use std::collections::HashMap;
use std::result;
use std::time::UNIX_EPOCH;

type Result<T> = result::Result<T, ScrobblerError>;

/// A Last.fm Scrobbler client. Submits song play information to Last.fm.
/// 
/// This is a client for the Scrobble and Now Playing endpoints on the Last.fm API. It handles API client and user 
/// auth, as well as providing Scrobble and Now Playing methods, plus support for sending batches of songs to Last.fm.
/// 
/// See the [official scrobbling API documentation](https://www.last.fm/api/scrobbling) for more information.
/// 
/// High-level example usage:
/// ```ignore
/// let username = "last-fm-username";
/// let password = "last-fm-password";
/// let api_key = "client-api-key";
/// let api_secret = "client-api-secret";
/// 
/// let mut scrobbler = Scrobbler.new(api_key, api_secret);
/// scrobbler.authenticate_with_password(username, password);
/// 
/// let song = Scrobble::new("Example Artist", "Example Song", "Example Album");
/// scrobbler.scrobble(song);
/// ```
pub struct Scrobbler {
    client: LastFm,
}

impl Scrobbler {

    /// Creates a new Scrobbler instance with the given Last.fm API Key and API Secret
    /// 
    /// # Usage
    /// ```ignore
    /// let api_secret = "xxx";
    /// let api_key = "123abc";
    /// let mut scrobbler = Scrobbler::new(api_key, api_secret);
    /// ...
    /// // Authenticate user with one of the available auth methods
    /// ```
    /// 
    /// # API Credentials
    /// All clients require the base API credentials: An API key and an API secret. These are obtained from Last.fm,
    /// and are specific to each *client*. These are credentials are totally separate from user authentication.
    /// 
    /// More information on authentication and API clients can be found in the Last.fm API documentation:
    /// 
    /// [API Authentication documentation](https://www.last.fm/api/authentication)
    /// 
    /// [API Account Registration form](https://www.last.fm/api/account/create)
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        let client = LastFm::new(api_key, api_secret);

        Self { client }
    }

    /// Authenticates a Last.fm user with the given username and password. 
    /// 
    /// This authentication path is known as the 'Mobile auth flow', but is valid for any platform. This is often the
    /// simplest method of authenticating a user with the API, requiring just username & password. Other Last.fm auth
    /// flows are available and might be better suited to your application, check the official Last.fm API docs for 
    /// further information.
    /// 
    /// # Usage
    /// ```ignore
    /// let mut scrobbler = Scrobbler::new(...)
    /// let username = "last-fm-user";
    /// let password = "hunter2";
    /// let response = scrobbler.authenticate_with_password(username, password);
    /// ...
    /// ```
    /// 
    /// # Last.fm API Documentation
    /// [Last.fm Mobile Auth Flow Documentation](https://www.last.fm/api/mobileauth)
    pub fn authenticate_with_password(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<SessionResponse> {
        self.client.set_user_credentials(username, password);
        Ok(self.client.authenticate_with_password()?)
    }

    /// Authenticates a Last.fm user with an authentication token. This method supports both the 'Web' and 'Desktop'
    /// Last.fm auth flows (check the API documentation to ensure you are using the correct authentication method for
    /// your needs).
    /// 
    /// # Usage
    /// ```ignore
    /// let mut scrobbler = Scrobbler.new(...);
    /// let auth_token = "token-from-last-fm";
    /// let response = scrobbler.authenticate_with_token(auth_token);
    /// ```
    /// 
    /// # Last.fm API Documentation
    /// [Last.fm Web Auth Flow Documentation](https://www.last.fm/api/webauth)
    /// 
    /// [Last.fm Desktop Auth Flow Documentation](https://www.last.fm/api/desktopauth)
    pub fn authenticate_with_token(&mut self, token: &str) -> Result<SessionResponse> {
        self.client.set_user_token(token);
        Ok(self.client.authenticate_with_token()?)
    }

    /// Authenticates a Last.fm user with a session key. 
    /// 
    /// # Usage
    /// ```ignore
    /// let mut scrobbler = Scrobbler::new(...);
    /// let session_key = "securely-saved-old-session-key";
    /// let response = scrobbler.authenticate_with_session_key(session_key);
    /// ```
    /// 
    /// # Response
    /// This method has no response: the crate expects a valid session key to be provided here and has no way to
    /// indicate if an invalidated key has been used. Clients will need to manually detect any authentication issues
    /// via API call error responses.
    /// 
    /// # A Note on Session Keys
    /// When authenticating successfully with username/password or with an authentication token (
    /// [`authenticate_with_password`] or [`authenticate_with_token`]), the Last.fm API will provide a Session Key.
    /// The Session Key is used internally to authenticate all subsequent requests to the Last.fm API. 
    /// 
    /// Session keys are valid _indefinitely_. Thus, they can be stored and used for authentication at a later time.
    /// A common pattern would be to authenticate initially via a username/password (or any other authentication flow)
    /// but store ONLY the session key (avoiding difficulties of securely storing usernames/passwords that can change 
    /// etc.) and use this method to authenticate all further sessions. The current session key can be fetched for 
    /// later use via [`Scrobbler::session_key`].
    /// 
    /// [`authenticate_with_password`]: struct.Scrobbler.html#method.authenticate_with_password
    /// [`authenticate_with_token`]: struct.Scrobbler.html#method.authenticate_with_token
    /// [`Scrobbler::session_key`]: struct.Scrobbler.html#method.session_key
    pub fn authenticate_with_session_key(&mut self, session_key: &str) {
        self.client.authenticate_with_session_key(session_key)
    }

    /// Registers the given [`Scrobble`]/track as the currently authenticated user's "now playing" track.
    /// 
    /// Most scrobbling clients will set the now-playing track as soon as the user starts playing it; this makes it 
    /// appear temporarily as the 'now listening' track on the user's profile. However use of this endpoint/method
    /// is entirely *optional* and can be skipped if you want.
    /// 
    /// # Usage
    /// This method behaves largely identically to the [`Scrobbler::scrobble`] method, just pointing to a different
    /// endpoint on the Last.fm API.
    /// 
    /// ```ignore
    /// let scrobbler = Scrobbler::new(...);
    /// // Scrobbler authentication ...
    /// let now_playing_track = Scrobble::new("Example Artist", "Example Track", "Example Album");
    /// match scrobbler.now_playing(now_playing_track) {
    ///     Ok(_) => println!("Now playing succeeded!"),
    ///     Err(err) => println("Now playing failed: {}", err)
    /// };
    /// ```
    /// 
    /// # Response
    /// On success a [`NowPlayingResponse`] is returned. This can often be ignored (as in the example code), but it
    /// contains information that may be of use to some clients. 
    /// 
    /// # Last.fm API Documentation
    /// [track.updateNowPlaying API Method Documentation](https://www.last.fm/api/show/track.updateNowPlaying)
    /// 
    /// [Now Playing Request Documentation](https://www.last.fm/api/scrobbling#now-playing-requests)
    /// 
    /// [`Scrobble`]: struct.Scrobble.html
    /// [`Scrobbler::scrobble`]: struct.Scrobbler.html#method.scrobble
    /// [`NowPlayingResponse`]: responses/struct.NowPlayingResponse.html
    pub fn now_playing(&self, scrobble: &Scrobble) -> Result<NowPlayingResponse> {
        let params = scrobble.as_map();

        Ok(self.client.send_now_playing(&params)?)
    }

    /// Registers a scrobble (play) of the given [`Scrobble`]/track.
    /// 
    /// # Usage
    /// Your [`Scrobbler`] must be fully authenticated before using [`Scrobbler::scrobble`].
    /// 
    /// ```ignore
    /// let scrobbler = Scrobbler::new(...);
    /// // Scrobbler authentication ...
    /// let scrobble_track = Scrobble::new("Example Artist", "Example Track", "Example Album");
    /// match scrobbler.scrobble(scrobble_track) {
    ///     Ok(_) => println!("Scrobble succeeded!"),
    ///     Err(err) => println("Scrobble failed: {}", err)
    /// };
    /// ```
    /// 
    /// # Response
    /// On success a [`ScrobbleResponse`] is returned. This can often be ignored (as in the example code), but it
    /// contains information that may be of use to some clients. 
    /// 
    /// # Last.fm API Documentation
    /// [track.scrobble API Method Documention](https://www.last.fm/api/show/track.scrobble)
    /// [Scrobble Request Documentation](https://www.last.fm/api/scrobbling#scrobble-requests)
    /// 
    /// [`Scrobble`]: struct.Scrobble.html
    /// [`Scrobbler`]: struct.Scrobbler.html
    /// [`Scrobbler::scrobble`]: struct.Scrobbler.html#method.scrobble
    /// [`ScrobbleResponse`]: responses/struct.ScrobbleResponse.html
    pub fn scrobble(&self, scrobble: &Scrobble) -> Result<ScrobbleResponse> {
        let mut params = scrobble.as_map();
        let current_time = UNIX_EPOCH.elapsed()?;

        params
            .entry("timestamp".to_string())
            .or_insert_with(|| format!("{}", current_time.as_secs()));

        Ok(self.client.send_scrobble(&params)?)
    }

    /// Registers a scrobble (play) of a collection of tracks. 
    /// 
    /// Takes a [`ScrobbleBatch`], effectively a wrapped `Vec<Scrobble>`, containing one or more [`Scrobble`] objects
    /// which are be submitted to the Scrobble endpoint in a single batch. 
    /// 
    /// # Usage
    /// Each [`ScrobbleBatch`] must contain 50 or fewer tracks. If a [`ScrobbleBatch`] containing more than 50
    /// [`Scrobble`]s is submitted an error will be returned. An error will similarly be returned if the batch contains
    /// no [`Scrobble`]s.
    /// 
    /// EXAMPLE CODE HERE TODO(v1)
    /// 
    /// # Response
    /// On success, returns a [`ScrobbleBatchResponse`]. This can be ignored by most clients, but contains some data
    /// that may be of interest.
    /// 
    /// # Last.fm API Documentation
    /// [track.scrobble API Method Documention](https://www.last.fm/api/show/track.scrobble)
    /// 
    /// [Scrobble Request Documentation](https://www.last.fm/api/scrobbling#scrobble-requests)
    /// 
    /// [`ScrobbleBatch`]: struct.ScrobbleBatch.html
    /// [`Scrobble`]: struct.Scrobble.html
    /// [`ScrobbleBatchResponse`]: responses/struct.ScrobbleBatchResponse.html
    pub fn scrobble_batch(&self, batch: &ScrobbleBatch) -> Result<BatchScrobbleResponse> {
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

            for (key, val) in &scrobble_params {
                // batched parameters need array notation suffix ie.
                // "artist[1]"" = "Artist 1", "artist[2]" = "Artist 2"
                params.insert(format!("{}[{}]", key, i), val.clone());
            }
        }

        Ok(self.client.send_batch_scrobbles(&params)?)
    }

    /// Gets the session key the client is currently authenticated with. Returns `None` if not authenticated. Valid
    /// session keys can be stored and used to authenticate with [`authenticate_with_session_key`].
    /// 
    /// See [`authenticate_with_session_key`] for more information on Last.fm API Session Keys
    /// 
    /// [`authenticate_with_session_key`]: struct.Scrobbler.html#method.authenticate_with_session_key
    pub fn session_key(&self) -> Option<&str> {
        self.client.session_key()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::error::Error;

    #[test]
    fn make_scrobbler_pass_auth() {
        let _m = mock("POST", mockito::Matcher::Any).create();

        let mut scrobbler = Scrobbler::new("api_key", "api_secret");
        let resp = scrobbler.authenticate_with_password("user", "pass");
        assert!(resp.is_err());

        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
                {   
                    "session": {
                        "key": "key",
                        "subscriber": 1337,
                        "name": "foo floyd"
                    }
                }
            "#,
            )
            .create();

        let resp = scrobbler.authenticate_with_password("user", "pass");
        assert!(resp.is_ok());
    }

    #[test]
    fn make_scrobbler_token_auth() {
        let _m = mock("POST", mockito::Matcher::Any).create();

        let mut scrobbler = Scrobbler::new("api_key", "api_secret");
        let resp = scrobbler.authenticate_with_token("some_token");
        assert!(resp.is_err());

        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
                {   
                    "session": {
                        "key": "key",
                        "subscriber": 1337,
                        "name": "foo floyd"
                    }
                }
            "#,
            )
            .create();

        let resp = scrobbler.authenticate_with_token("some_token");
        assert!(resp.is_ok());
    }

    #[test]
    fn check_scrobbler_error() {
        let err = ScrobblerError::new("test_error".into());
        let fmt = format!("{}", err);
        assert_eq!("test_error", fmt);

        let desc = err.description();
        assert_eq!("test_error", desc);

        assert!(err.source().is_none());
    }

    #[test]
    fn check_scrobbler_now_playing() {
        let mut scrobbler = Scrobbler::new("api_key", "api_secret");

        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
                {   
                    "session": {
                        "key": "key",
                        "subscriber": 1337,
                        "name": "foo floyd"
                    }
                }
            "#,
            )
            .create();

        let resp = scrobbler.authenticate_with_token("some_token");
        assert!(resp.is_ok());

        let mut scrobble = crate::models::metadata::Scrobble::new(
            "foo floyd and the fruit flies",
            "old bananas",
            "old bananas",
        );
        scrobble.with_timestamp(1337);

        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
            { 
                "nowplaying": {
                            "artist": [ "0", "foo floyd and the fruit flies" ],
                            "album": [ "1", "old bananas" ], 
                            "albumArtist": [ "0", "foo floyd"],
                            "track": [ "1", "old bananas"], 
                            "timestamp": "2019-10-04 13:23:40" 
                        }
            }
            "#,
            )
            .create();

        let resp = scrobbler.now_playing(&scrobble);
        assert!(resp.is_ok());
    }

    #[test]
    fn check_scrobbler_scrobble() {
        let mut scrobbler = Scrobbler::new("api_key", "api_secret");

        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
                {   
                    "session": {
                        "key": "key",
                        "subscriber": 1337,
                        "name": "foo floyd"
                    }
                }
            "#,
            )
            .create();

        let resp = scrobbler.authenticate_with_token("some_token");
        assert!(resp.is_ok());

        let mut scrobble = crate::models::metadata::Scrobble::new(
            "foo floyd and the fruit flies",
            "old bananas",
            "old bananas",
        );
        scrobble.with_timestamp(1337);

        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
            { 
                "scrobbles": [{
                        "artist": [ "0", "foo floyd and the fruit flies" ],
                        "album": [ "1", "old bananas" ], 
                        "albumArtist": [ "0", "foo floyd"],
                        "track": [ "1", "old bananas"], 
                        "timestamp": "2019-10-04 13:23:40" 
                }]
            }
            "#,
            )
            .create();

        let resp = scrobbler.scrobble(&scrobble);
        assert!(resp.is_ok());
    }
}
