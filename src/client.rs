// Last.fm scrobble API 2.0 client
use std::collections::HashMap;
use std::fmt;
use ureq;

use crate::auth::Credentials;
use crate::models::responses::{
    AuthResponse, BatchScrobbleResponse, BatchScrobbleResponseWrapper, NowPlayingResponse,
    NowPlayingResponseWrapper, ScrobbleResponse, ScrobbleResponseWrapper, SessionResponse,
};

pub enum ApiOperation {
    AuthWebSession,
    AuthMobileSession,
    NowPlaying,
    Scrobble,
}

impl fmt::Display for ApiOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match *self {
            Self::AuthWebSession => "auth.getSession",
            Self::AuthMobileSession => "auth.getMobileSession",
            Self::NowPlaying => "track.updateNowPlaying",
            Self::Scrobble => "track.scrobble",
        };
        write!(f, "{}", str)
    }
}

pub struct LastFm {
    auth: Credentials,
    http_client: ureq::Agent,
}

impl LastFm {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        let partial_auth = Credentials::new_partial(api_key, api_secret);
        let http_client = ureq::agent();

        Self {
            auth: partial_auth,
            http_client,
        }
    }

    pub fn set_user_credentials(&mut self, username: &str, password: &str) {
        self.auth.set_user_credentials(username, password);
    }

    pub fn set_user_token(&mut self, token: &str) {
        self.auth.set_user_token(token);
    }

    pub fn authenticate_with_password(&mut self) -> Result<SessionResponse, String> {
        let params = self.auth.get_auth_request_params()?;

        let body = self
            .api_request(&ApiOperation::AuthMobileSession, params)
            .map_err(|msg| format!("Authentication failed: {}", msg))?;

        let decoded: AuthResponse = serde_json::from_str(body.as_str())
            .map_err(|err| format!("Authentication failed: {}", err))?;

        self.auth.set_session_key(&decoded.session.key);

        Ok(decoded.session)
    }

    pub fn authenticate_with_token(&mut self) -> Result<SessionResponse, String> {
        let params = self.auth.get_auth_request_params()?;

        let body = self
            .api_request(&ApiOperation::AuthWebSession, params)
            .map_err(|msg| format!("Authentication failed: {}", msg))?;

        let decoded: AuthResponse = serde_json::from_str(body.as_str())
            .map_err(|err| format!("Authentication failed: {}", err))?;

        self.auth.set_session_key(&decoded.session.key);

        Ok(decoded.session)
    }

    /// Authenticates with a session key 
    /// 
    /// This requires no initial authentication with the API, so we simply store the key. It must be a valid session
    /// key. Session keys are documented at `Scrobbler::authenticate_with_session_key`.
    pub fn authenticate_with_session_key(&mut self, session_key: &str) {
        self.auth.set_session_key(session_key)
    }

    pub fn session_key(&self) -> Option<&str> {
        self.auth.session_key()
    }

    pub fn send_now_playing(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<NowPlayingResponse, String> {
        let body = self
            .send_authenticated_request(&ApiOperation::NowPlaying, params)
            .map_err(|msg| format!("Now playing request failed: {}", msg))?;

        let decoded: NowPlayingResponseWrapper = serde_json::from_str(body.as_str())
            .map_err(|msg| format!("Now playing request failed: {}", msg))?;

        Ok(decoded.nowplaying)
    }

    pub fn send_scrobble(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<ScrobbleResponse, String> {
        let body = self
            .send_authenticated_request(&ApiOperation::Scrobble, params)
            .map_err(|msg| format!("Scrobble request failed: {}", msg))?;

        let decoded: ScrobbleResponseWrapper = serde_json::from_str(body.as_str())
            .map_err(|msg| format!("Scrobble request failed: {}", msg))?;

        Ok(decoded.scrobbles.scrobble)
    }

    pub fn send_batch_scrobbles(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<BatchScrobbleResponse, String> {
        let body = self
            .send_authenticated_request(&ApiOperation::Scrobble, params)
            .map_err(|msg| format!("Batch scrobble request failed: {}", msg))?;

        let wrapper: BatchScrobbleResponseWrapper = serde_json::from_str(body.as_str())
            .map_err(|msg| format!("Batch scrobble request failed: {}", msg))?;

        Ok(BatchScrobbleResponse {
            scrobbles: wrapper.scrobbles.scrobbles,
        })
    }

    pub fn send_authenticated_request(
        &self,
        operation: &ApiOperation,
        params: &HashMap<String, String>,
    ) -> Result<String, String> {
        if !self.auth.is_authenticated() {
            return Err("Not authenticated".to_string());
        }

        let mut req_params = self.auth.get_request_params();
        for (k, v) in params {
            req_params.insert(k.clone(), v.clone());
        }

        self.api_request(&operation, req_params)
    }

    fn api_request(
        &self,
        operation: &ApiOperation,
        params: HashMap<String, String>,
    ) -> Result<String, String> {
        let resp = self
            .send_request(&operation, params)
            .map_err(|err| err.to_string())?;

        if resp.error() {
            return Err(format!("Non Success status ({})", resp.status()));
        }

        let resp_body = resp
            .into_string()
            .map_err(|_| "Failed to read response body".to_string())?;

        Ok(resp_body)
    }

    fn send_request(
        &self,
        operation: &ApiOperation,
        mut params: HashMap<String, String>,
    ) -> Result<ureq::Response, String> {
        #[cfg(not(test))]
        let url = "https://ws.audioscrobbler.com/2.0/?format=json";
        #[cfg(test)]
        let url = &mockito::server_url();

        let signature = self.auth.get_signature(operation.to_string(), &params);

        params.insert("method".to_string(), operation.to_string());
        params.insert("api_sig".to_string(), signature);

        let params: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        let resp = self.http_client.post(url).send_form(&params[..]);
        match resp.synthetic_error() {
            None => Ok(resp),
            Some(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn check_send_api_requests() {
        let _m = mock("POST", mockito::Matcher::Any)
            .match_body(mockito::Matcher::Any)
            .create();
        let mut client = LastFm::new("key", "secret");
        client.auth.set_user_credentials("username", "password");
        let params = client.auth.get_auth_request_params().unwrap();

        let resp = client.api_request(&ApiOperation::AuthWebSession, params.clone());
        assert!(resp.is_ok());
        let resp = client.api_request(&ApiOperation::AuthMobileSession, params.clone());
        assert!(resp.is_ok());
        let resp = client.api_request(&ApiOperation::Scrobble, params.clone());
        assert!(resp.is_ok());
        let resp = client.api_request(&ApiOperation::NowPlaying, params.clone());
        assert!(resp.is_ok());

        // authenticated request
        let resp = client.send_authenticated_request(&ApiOperation::NowPlaying, &params);
        assert!(resp.is_err());
        client.auth.set_session_key("sesh");
        let resp = client.send_authenticated_request(&ApiOperation::NowPlaying, &params);
        assert!(resp.is_ok());
    }

    #[test]
    fn check_send_scrobble() {
        let _m = mock("POST", mockito::Matcher::Any).create();

        let mut client = LastFm::new("key", "secret");
        client.auth.set_user_credentials("username", "password");
        client.auth.set_session_key("SeshKey");
        let params = client.auth.get_auth_request_params().unwrap();

        let resp = client.send_scrobble(&params);
        assert!(resp.is_err());

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

        let resp = client.send_scrobble(&params);
        assert!(resp.is_ok());
    }

    #[test]
    fn check_send_batch_scrobble() {
        let _m = mock("POST", mockito::Matcher::Any).create();

        let mut client = LastFm::new("key", "secret");
        client.auth.set_user_credentials("username", "password");
        client.auth.set_session_key("SeshKey");
        let params = client.auth.get_auth_request_params().unwrap();

        let resp = client.send_batch_scrobbles(&params);
        assert!(resp.is_err());

        // Test with parsing single-scrobble response
        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
            { 
                "scrobbles": {
                    "scrobble":
                        {
                            "artist": [ "0", "foo floyd and the fruit flies" ],
                            "album": [ "1", "old bananas" ], 
                            "albumArtist": [ "0", "foo floyd"],
                            "track": [ "1", "old bananas"], 
                            "timestamp": "2019-10-04 13:23:40" 
                        }
                }
            }
            "#,
            )
            .create();

        let resp = client.send_batch_scrobbles(&params);
        assert!(resp.is_ok());

        // Test with parsing multi-scrobble response
        let _m = mock("POST", mockito::Matcher::Any)
            .with_body(
                r#"
            { 
                "scrobbles": {
                    "scrobble":[
                        {
                            "artist": [ "0", "foo floyd and the fruit flies" ],
                            "album": [ "1", "old bananas" ], 
                            "albumArtist": [ "0", "foo floyd"],
                            "track": [ "1", "old bananas"], 
                            "timestamp": "2019-10-04 13:23:40" 
                        },
                        {
                            "artist": [ "0", "foo floyd and the fruit flies" ],
                            "album": [ "1", "old bananas" ], 
                            "albumArtist": [ "0", "foo floyd"],
                            "track": [ "1", "old bananas"], 
                            "timestamp": "2019-10-04 13:23:40" 
                        }
                    ]
                }
            }
            "#,
            )
            .create();

        let resp = client.send_batch_scrobbles(&params);
        assert!(resp.is_ok());
    }

    #[test]
    fn check_send_now_playing() {
        let _m = mock("POST", mockito::Matcher::Any).create();

        let mut client = LastFm::new("key", "secret");
        client.auth.set_user_credentials("username", "password");
        client.auth.set_session_key("SeshKey");
        let params = client.auth.get_auth_request_params().unwrap();

        let resp = client.send_now_playing(&params);
        assert!(resp.is_err());

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

        let resp = client.send_now_playing(&params);
        assert!(resp.is_ok());
    }

    #[test]
    fn check_set_user_creds_and_token_then_auth() {
        let mut client = LastFm::new("key", "secret");
        client.set_user_credentials("user", "pass");
        client.set_user_token("SomeToken");

        let _m = mock("POST", mockito::Matcher::Any).create();

        let res = client.authenticate_with_password();
        assert!(res.is_err());

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

        let res = client.authenticate_with_password();
        assert!(res.is_ok());
    }

    #[test]
    fn check_session_key_authentication() {
        let mut client = LastFm::new("key", "secret");
        client.set_user_credentials("user", "pass");
        client.authenticate_with_session_key("seshkey");
        assert_eq!("seshkey", client.session_key().unwrap());
    }
}
