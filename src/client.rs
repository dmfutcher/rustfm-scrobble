// Last.fm scrobble API 2.0 client

use auth::{AuthCredentials};

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

    pub fn send_authentication_request(&self, object: String) -> Result<(), &'static str> {
        if !self.auth.is_valid() {
            return Err("Invalid authentication parameters")
        }

        self.send_request(object)
    }

    pub fn send_authenticated_request(&self, object: String) -> Result<(), &'static str> {
        if !self.auth.is_authenticated() {
            return Err("Not authenticated")
        }

        self.send_request(object)
    }

    fn send_request(&self, object: String) -> Result<(), &'static str> {
        Ok(())
    }

}
