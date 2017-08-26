// Last.fm scrobble API 2.0 client

use std::collections::HashMap;
use std::io::Read;
use reqwest;
use reqwest::{Client, StatusCode};
use serde_json;

use auth::AuthCredentials;
use dto::{
    AuthResponse, 
    SessionResponse,
    NowPlayingResponse,
    NowPlayingResponseWrapper,
    ScrobbleResponse,
    ScrobbleResponseWrapper
};

pub enum ApiOperation {
    AuthSession,
    NowPlaying,
    Scrobble
}

impl ApiOperation {

    fn to_string(&self) -> String {
        match *self {
            ApiOperation::AuthSession => "auth.getMobileSession",
            ApiOperation::NowPlaying => "track.updateNowPlaying",
            ApiOperation::Scrobble => "track.scrobble"
        }.to_string()
    }

}

pub struct LastFmClient {
    auth: AuthCredentials,
    http_client: Client
}

impl LastFmClient {

    pub fn new(api_key: String, api_secret: String) -> LastFmClient {
        let partial_auth = AuthCredentials::new_partial(api_key, api_secret);
        let http_client = Client::new().unwrap();

        LastFmClient{
            auth: partial_auth,
            http_client: http_client
        }
    }

    pub fn set_user_credentials(&mut self, username: String, password: String) {
        self.auth.set_user_credentials(username, password);
    }

    pub fn authenticate(&mut self) -> Result<SessionResponse, String> {
        if !self.auth.is_valid() {
            return Err("Invalid authentication parameters".to_string())
        }

        let params = self.auth.get_auth_request_params();

        match self.api_request(ApiOperation::AuthSession, params) {
            Ok(body) => {
                let decoded: AuthResponse = serde_json::from_str(body.as_str()).unwrap();
                self.auth.set_session_key(decoded.session.clone().key);

                Ok(decoded.session)
            },
            Err(msg) => Err(format!("Authentication failed: {}", msg))
        }
    }

    pub fn send_now_playing(&self, params: &HashMap<&str, String>) -> Result<NowPlayingResponse, String> {
        match self.send_authenticated_request(ApiOperation::NowPlaying, params) {
            Ok(body) => {
                let decoded: NowPlayingResponseWrapper = serde_json::from_str(body.as_str()).unwrap();
                Ok(decoded.nowplaying)
            },
            Err(msg) => Err(format!("Now playing request failed: {}", msg))
        }
    }

    pub fn send_scrobble(&self, params: &HashMap<&str, String>) -> Result<ScrobbleResponse, String> {
        match self.send_authenticated_request(ApiOperation::Scrobble, params) {
            Ok(body) => {
                let decoded: ScrobbleResponseWrapper = serde_json::from_str(body.as_str()).unwrap();
                Ok(decoded.scrobbles.scrobble)
            },
            Err(msg) => Err(format!("Scrobble request failed: {}", msg))
        }
    }

    pub fn send_authenticated_request(&self, operation: ApiOperation, params: &HashMap<&str, String>) -> Result<String, String> {
        if !self.auth.is_authenticated() {
            return Err("Not authenticated".to_string())
        }

        let mut req_params = self.auth.get_request_params();
        for (k, v) in params {
            req_params.insert(k, v.clone());
        }

        self.api_request(operation, req_params)
    }

    fn api_request(&self, operation: ApiOperation, params: HashMap<&str, String>) -> Result<String, String> {            
        match self.send_request(operation, params) {
            Ok(mut resp) => {
                let status = resp.status();
                if status != StatusCode::Ok {
                    return Err(format!("Non Success status ({})", status));
                }

                let mut resp_body = String::new();
                match resp.read_to_string(&mut resp_body) {
                    Ok(_) => return Ok(resp_body),
                    Err(_) => return Err("Failed to read response body".to_string())
                }
            },
            Err(msg) => return Err(format!("{}", msg))
        }
    }

    fn send_request(&self, operation: ApiOperation, params: HashMap<&str, String>) -> Result<reqwest::Response, reqwest::Error> {
        let url = "https://ws.audioscrobbler.com/2.0/?format=json";
        let signature = self.auth.get_signature(operation.to_string(), &params);

        let mut req_params = params.clone();
        req_params.insert("method", operation.to_string());
        req_params.insert("api_sig", signature);

        self.http_client
            .post(url)?
            .form(&req_params)?
            .send()
    }

}
