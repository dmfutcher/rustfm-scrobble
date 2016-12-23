// Authentication utilities for Last.fm Scrobble API 2.0

pub struct AuthCredentials {
    // Application specific key & secret
    api_key: String,
    api_secret: String,

    // Individual user's username & pass
    username: String,
    password: String
}

impl AuthCredentials {

    pub fn new_partial(api_key: String, api_secret: String) -> AuthCredentials {
        AuthCredentials{
            api_key: api_key,
            api_secret: api_secret,

            username: String::new(),
            password: String::new()
        }
    }

    pub fn is_valid(&self) -> bool {
        self.api_key.len() > 0 && self.api_secret.len() > 0 && self.username.len() > 0
            && self.password.len() > 0
    }

}
