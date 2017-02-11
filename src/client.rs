// Last.fm scrobble API 2.0 client

use std::collections::HashMap;

use auth::AuthCredentials;

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

    pub fn send_authentication_request(&self) -> Result<(), &'static str> {
        if !self.auth.is_valid() {
            return Err("Invalid authentication parameters")
        }

        let params = self.auth.get_auth_request_params();

        self.send_request("auth.getMobileSession", params)
    }

    pub fn send_authenticated_request(&self, object: &str) -> Result<(), &'static str> {
        if !self.auth.is_authenticated() {
            return Err("Not authenticated")
        }

        self.send_request(object, HashMap::new())
    }

    fn send_request(&self, object: &str, params: HashMap<&str, String>) -> Result<(), &'static str> {
        let signature = self.auth.get_signature(object, params);
        Ok(())
    }


}
