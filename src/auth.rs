// Authentication utilities for Last.fm Scrobble API 2.0
use std::collections::HashMap;

pub struct AuthCredentials {
    // Application specific key & secret
    api_key: String,
    api_secret: String,

    // Individual user's username & pass, or auth token
    credentials: Option<Credentials>,

    // Long-lasting session key (used once UserCredentials are authenticated)
    session_key: Option<String>,
}

#[derive(Clone)]
struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Clone)]
enum Credentials {
    UserSupplied(UserCredentials),
    Token(String),
}

impl UserCredentials {
    pub fn can_authenticate(&self) -> bool {
        !self.username.is_empty() && !self.password.is_empty()
    }
}

impl AuthCredentials {
    pub fn new_partial(api_key: &str, api_secret: &str) -> AuthCredentials {
        AuthCredentials {
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
            credentials: None,
            session_key: None,
        }
    }

    pub fn set_user_credentials(&mut self, username: &str, password: &str) {
        self.credentials = Some(Credentials::UserSupplied(UserCredentials {
            username: username.to_owned(),
            password: password.to_owned(),
        }));

        // Invalidate session because we have new credentials
        self.session_key = None
    }

    pub fn set_user_token(&mut self, token: &str) {
        self.credentials = Some(Credentials::Token(token.to_owned()));

        // Invalidate session because we have new credentials
        self.session_key = None
    }

    pub fn set_session_key(&mut self, key: &str) {
        self.session_key = Some(key.to_owned());
    }

    pub fn session_key(&self) -> Option<&str> {
        self.session_key.as_ref().map(std::ops::Deref::deref)
    }

    // Returns true if we have valid authentication parameters AND a session token
    pub fn is_authenticated(&self) -> bool {
        self.session_key.is_some()
    }

    pub fn get_auth_request_params(&self) -> Result<HashMap<String, String>, String> {
        let credentials = self
            .credentials
            .as_ref()
            .ok_or("No user credentials available")?;

        if self.api_key.is_empty() || self.api_secret.is_empty() {
            return Err("Invalid authentication parameters".to_string());
        }

        let mut params = HashMap::new();
        params.insert("api_key".to_string(), self.api_key.clone());

        match credentials {
            Credentials::UserSupplied(user_credentials) => {
                if !user_credentials.can_authenticate() {
                    return Err("Invalid authentication credentials".to_string());
                }
                params.insert("username".to_string(), user_credentials.username.clone());
                params.insert("password".to_string(), user_credentials.password.clone());
            }
            Credentials::Token(token) => {
                params.insert("token".to_string(), token.clone());
            }
        }

        Ok(params)
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

        format!("{:x}", md5::compute(sig.as_bytes()))
    }
}
