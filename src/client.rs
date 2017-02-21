// Last.fm scrobble API 2.0 client

use std::collections::HashMap;
use std::io::Read;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json;

use auth::AuthCredentials;
use dto::AuthResponseDto;

pub struct LastFmClient {
    auth: AuthCredentials
}

impl LastFmClient {

    pub fn new(api_key: String, api_secret: String) -> LastFmClient {
        let partial_auth = AuthCredentials::new_partial(api_key, api_secret);

        LastFmClient{
            auth: partial_auth
        }
    }

    pub fn set_user_credentials(&mut self, username: String, password: String) {
        self.auth.set_user_credentials(username, password);
    }

    pub fn send_authentication_request(&mut self) -> Result<(), String> {
        if !self.auth.is_valid() {
            return Err("Invalid authentication parameters".to_string())
        }

        let params = self.auth.get_auth_request_params();

        match self.send_request("auth.getMobileSession", params) {
            Ok(body) => {
                let decoded: AuthResponseDto = serde_json::from_str(body.as_str()).unwrap();
                self.auth.set_session_key(decoded.session.key);

                Ok(())
            },
            Err(msg) => Err(format!("Authentication failed: {}", msg))
        }
    }

    pub fn send_authenticated_request(&self, object: &str) -> Result<String, String> {
        if !self.auth.is_authenticated() {
            return Err("Not authenticated".to_string())
        }

        self.send_request(object, HashMap::new())
    }

    fn send_request(&self, object: &str, params: HashMap<&str, String>) -> Result<String, String> {
        let url = "https://ws.audioscrobbler.com/2.0/?format=json";
        let mut url_params = params.clone();
        let signature = self.auth.get_signature(object, params);
        url_params.insert("method", object.to_string());
        url_params.insert("api_sig", signature);

        let mut body = String::new();
        for (k, v) in &url_params {
            body.push_str((format!("{}={}&", k, v.as_str())).as_str());
        }

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let result = client
            .post(url)
            .body(body.as_str())
            .send();

        match result {
            Ok(mut resp) => {
                if resp.status != StatusCode::Ok {
                    return Err(format!("Non Success status ({})", resp.status));
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

}
