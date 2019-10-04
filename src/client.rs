// Last.fm scrobble API 2.0 client

use reqwest;
use reqwest::{Client, StatusCode};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;

#[cfg(test)]
use mockito;

use crate::auth::AuthCredentials;
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
            ApiOperation::AuthWebSession => "auth.getSession",
            ApiOperation::AuthMobileSession => "auth.getMobileSession",
            ApiOperation::NowPlaying => "track.updateNowPlaying",
            ApiOperation::Scrobble => "track.scrobble",
        };
        write!(f, "{}", str)
    }
}

pub struct LastFmClient {
    auth: AuthCredentials,
    http_client: Client,
}

impl LastFmClient {
    pub fn new(api_key: &str, api_secret: &str) -> LastFmClient {
        let partial_auth = AuthCredentials::new_partial(api_key, api_secret);
        let http_client = Client::new();

        LastFmClient {
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
            .api_request(ApiOperation::AuthMobileSession, params)
            .map_err(|msg| format!("Authentication failed: {}", msg))?;

        let decoded: AuthResponse = serde_json::from_str(body.as_str())
            .map_err(|err| format!("Authentication failed: {}", err))?;

        self.auth.set_session_key(&decoded.session.key);

        Ok(decoded.session)
    }

    pub fn authenticate_with_token(&mut self) -> Result<SessionResponse, String> {
        let params = self.auth.get_auth_request_params()?;

        let body = self
            .api_request(ApiOperation::AuthWebSession, params)
            .map_err(|msg| format!("Authentication failed: {}", msg))?;

        let decoded: AuthResponse = serde_json::from_str(body.as_str())
            .map_err(|err| format!("Authentication failed: {}", err))?;

        self.auth.set_session_key(&decoded.session.key);

        Ok(decoded.session)
    }

    pub fn authenticate_with_session_key(&mut self, session_key: &str) {
        // TODO: How to verify session key at this point?
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
            .send_authenticated_request(ApiOperation::NowPlaying, params)
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
            .send_authenticated_request(ApiOperation::Scrobble, params)
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
            .send_authenticated_request(ApiOperation::Scrobble, params)
            .map_err(|msg| format!("Batch scrobble request failed: {}", msg))?;

        let wrapper: BatchScrobbleResponseWrapper = serde_json::from_str(body.as_str())
            .map_err(|msg| format!("Batch scrobble request failed: {}", msg))?;

        Ok(BatchScrobbleResponse {
            scrobbles: wrapper.scrobbles.scrobbles,
        })
    }

    pub fn send_authenticated_request(
        &self,
        operation: ApiOperation,
        params: &HashMap<String, String>,
    ) -> Result<String, String> {
        if !self.auth.is_authenticated() {
            return Err("Not authenticated".to_string());
        }

        let mut req_params = self.auth.get_request_params();
        for (k, v) in params {
            req_params.insert(k.clone(), v.clone());
        }

        self.api_request(operation, req_params)
    }

    fn api_request(
        &self,
        operation: ApiOperation,
        params: HashMap<String, String>,
    ) -> Result<String, String> {
        let mut resp = self
            .send_request(operation, params)
            .map_err(|err| err.to_string())?;

        let status = resp.status();
        if status != StatusCode::OK {
            return Err(format!("Non Success status ({})", status));
        }

        let mut resp_body = String::new();
        resp.read_to_string(&mut resp_body)
            .map_err(|_| "Failed to read response body".to_string())?;

        Ok(resp_body)
    }

    fn send_request(
        &self,
        operation: ApiOperation,
        mut params: HashMap<String, String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        #[cfg(not(test))]
        let url = "https://ws.audioscrobbler.com/2.0/?format=json";
        #[cfg(test)]
        let url = &mockito::server_url();

        let signature = self.auth.get_signature(operation.to_string(), &params);

        params.insert("method".to_string(), operation.to_string());
        params.insert("api_sig".to_string(), signature);

        self.http_client.post(url).form(&params).send()
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
        let mut client = LastFmClient::new("key", "secret");
        client.auth.set_user_credentials("username", "password");
        let params = client.auth.get_auth_request_params().unwrap();

        let resp = client.api_request(ApiOperation::AuthWebSession, params.clone());
        assert!(resp.is_ok());
        let resp = client.api_request(ApiOperation::AuthMobileSession, params.clone());
        assert!(resp.is_ok());
        let resp = client.api_request(ApiOperation::Scrobble, params.clone());
        assert!(resp.is_ok());
        let resp = client.api_request(ApiOperation::NowPlaying, params.clone());
        assert!(resp.is_ok());

        // authenticated request
        let resp = client.send_authenticated_request(ApiOperation::NowPlaying, &params);
        assert!(resp.is_err());
        client.auth.set_session_key("sesh");
        let resp = client.send_authenticated_request(ApiOperation::NowPlaying, &params);
        assert!(resp.is_ok());
    }

    // this test sucks and I need to fix it before I submit the PR
    #[test]
    fn check_scrobble_requests() {
        // scrobbles / now playing
        let _m = mock("POST", mockito::Matcher::Any)
            .with_body("{ \"scrobble\": { \"artist\": \"foo floyd and the fruit flies\", \"track\": \"old bananas\", \"album\": \"old bananas\", \"timestamp\": \"2019-10-04 13:23:40\" } }")
            .create();

        let mut client = LastFmClient::new("key", "secret");
        client.auth.set_user_credentials("username", "password");
        let params = client.auth.get_auth_request_params().unwrap();

        let resp = client.send_scrobble(&params);
        assert!(resp.is_err()); // lets defuckulate the api responses later
        let resp = client.send_batch_scrobbles(&params);
        assert!(resp.is_err());
        let resp = client.send_now_playing(&params);
        assert!(resp.is_err());
    }

    #[test]
    fn check_set_user_creds() {
        let mut client = LastFmClient::new("key", "secret");
        client.set_user_credentials("user", "pass");
        // looks like everything I need from here
        // for this test is private...
        // Will finish later.
    }

    #[test]
    fn sesh_keys() {
        let mut client = LastFmClient::new("key", "secret");
        client.set_user_credentials("user", "pass");
        client.authenticate_with_session_key("seshkey");
        assert_eq!("seshkey", client.session_key().unwrap());
    }
}
