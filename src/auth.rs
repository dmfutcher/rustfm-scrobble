// Authentication utilities for Last.fm Scrobble API 2.0
use std::collections::HashMap;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub struct AuthCredentials {
    // Application specific key & secret
    api_key: String,
    api_secret: String,

    // Individual user's username & pass
    username: String,
    password: String,

    // Dynamic parameter not included until we're authenticated
    session_key: Option<String>,
}

impl AuthCredentials {
    pub fn new_partial(api_key: String, api_secret: String) -> AuthCredentials {
        AuthCredentials {
            api_key: api_key,
            api_secret: api_secret,

            username: String::new(),
            password: String::new(),

            session_key: None,
        }
    }

    pub fn set_user_credentials(&mut self, username: String, password: String) {
        self.username = username;
        self.password = password;

        // Invalidate session because we have new credentials
        self.session_key = None
    }

    pub fn set_session_key(&mut self, key: String) {
        self.session_key = Some(key);
    }

    // Returns true if there's enough valid data to attempt authentication (ignores session key)
    pub fn is_valid(&self) -> bool {
        !self.api_key.is_empty() && !self.api_secret.is_empty() && !self.username.is_empty() &&
        !self.password.is_empty()
    }

    // Returns true if we have valid authentication parameters AND a session token
    pub fn is_authenticated(&self) -> bool {
        self.is_valid() && self.session_key.is_some()
    }

    pub fn get_auth_request_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("username".to_string(), self.username.clone());
        params.insert("password".to_string(), self.password.clone());
        params.insert("api_key".to_string(), self.api_key.clone());

        params
    }

    pub fn get_request_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("api_key".to_string(), self.api_key.clone());
        params.insert("sk".to_string(), self.session_key.clone().unwrap());

        params
    }

    pub fn get_signature(&self, method: String, params: &HashMap<String, String>) -> String {
        let mut sig_params = params.clone();
        sig_params.insert("method".to_string(), method);

        let mut keys = Vec::new();
        for k in sig_params.keys() {
            keys.push(k);
        }

        keys.sort();

        let mut sig = String::new();
        for k in keys {
            sig.push_str((k.to_string() + sig_params[k].as_str()).as_str())
        }

        sig.push_str(self.api_secret.as_str());

        let mut hash = Md5::new();
        hash.input(sig.as_bytes());
        hash.result_str()
    }
}
