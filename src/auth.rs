// Authentication utilities for Last.fm Scrobble API 2.0
use std::collections::HashMap;

pub struct AuthCredentials {
    // Application specific key & secret
    api_key: String,
    api_secret: String,

    // Individual user's username & pass
    username: String,
    password: String,

    // Dynamic parameter not included until we're authenticated
    session_key: Option<String>
}

impl AuthCredentials {

    pub fn new_partial(api_key: String, api_secret: String) -> AuthCredentials {
        AuthCredentials{
            api_key: api_key,
            api_secret: api_secret,

            username: String::new(),
            password: String::new(),

            session_key: None
        }
    }

    pub fn set_user_credentials(&mut self, username: String, password: String) {
        self.username = username;
        self.password = password;

        // Invalidate session because we have new credentials
        self.session_key = None
    }

    // Returns true if there's enough valid data to attempt authentication (ignores session key)
    pub fn is_valid(&self) -> bool {
        self.api_key.len() > 0 && self.api_secret.len() > 0 && self.username.len() > 0
            && self.password.len() > 0
    }

    // Returns true if we have valid authentication parameters AND a session token
    pub fn is_authenticated(&self) -> bool {
        self.is_valid() && self.session_key.is_some()
    }

    pub fn get_auth_request_params(&self) -> HashMap<&str, String> {
        let mut params = HashMap::new();
        params.insert("username", self.username.clone());
        params.insert("password", self.password.clone());
        params.insert("api_key", self.api_key.clone());

        return params
    }

    pub fn get_signature(&self, method: &str, params: HashMap<&str, String>) -> String {
        let mut sig_params = params.clone();
        sig_params.insert("method", method.to_string());

        let mut sig = String::new();
        for (k, v) in &sig_params {
            sig.push_str((k.to_string() + v).as_str())
        }

        println!("{}", sig);
        sig
    }

}
