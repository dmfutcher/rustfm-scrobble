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

    pub fn send_request(&self, object: String) -> Result<(), &'static str> {
        if !self.auth.is_valid() {
            return Err("Invalid authentication credentials")
        }

        Ok(())
    }

}
