// Authentication utilities for Last.fm Scrobble API 2.0
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Credentials {
    // Application specific key & secret
    api_key: String,
    api_secret: String,

    // Individual user's username & pass, or auth token
    credentials: Option<CredentialsVariant>,

    // Long-lasting session key (used once UserCredentials are authenticated)
    session_key: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Clone, Debug, PartialEq)]
enum CredentialsVariant {
    UserSupplied(UserCredentials),
    Token(String),
}

impl UserCredentials {

    /// Returns true when a valid username & password are set
    pub fn can_authenticate(&self) -> bool {
        !self.username.is_empty() && !self.password.is_empty()
    }

}

impl Credentials {
    pub fn new_partial(api_key: &str, api_secret: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
            credentials: None,
            session_key: None,
        }
    }

    pub fn set_user_credentials(&mut self, username: &str, password: &str) {
        self.credentials = Some(CredentialsVariant::UserSupplied(UserCredentials {
            username: username.to_owned(),
            password: password.to_owned(),
        }));

        // Invalidate session because we have new credentials
        self.clear_session_key()
    }

    pub fn set_user_token(&mut self, token: &str) {
        self.credentials = Some(CredentialsVariant::Token(token.to_owned()));
        self.clear_session_key()
    }

    // Invalidates session. Usually because we have new user token / credentials, which invalidates
    // the current session.
    fn clear_session_key(&mut self) {
        self.session_key = None
    }

    pub fn set_session_key(&mut self, key: &str) {
        self.session_key = Some(key.to_owned());
    }

    pub fn session_key(&self) -> Option<&str> {
        self.session_key.as_ref().map(std::ops::Deref::deref)
    }

    // Returns true if we are currently authenticated (have a valid session token set)
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
            CredentialsVariant::UserSupplied(user_credentials) => {
                if !user_credentials.can_authenticate() {
                    return Err("Invalid authentication credentials".to_string());
                }
                params.insert("username".to_string(), user_credentials.username.clone());
                params.insert("password".to_string(), user_credentials.password.clone());
            }
            CredentialsVariant::Token(token) => {
                params.insert("token".to_string(), token.clone());
            }
        }

        Ok(params)
    }

    pub fn get_request_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("api_key".to_string(), self.api_key.clone());
        params.insert(
            "sk".to_string(),
            self.session_key.clone().unwrap_or_default(),
        );

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_user_credentials() {
        let empty = UserCredentials {
            username: "".into(),
            password: "".into(),
        };

        assert!(!UserCredentials::can_authenticate(&empty));

        let not_empty = UserCredentials {
            username: "foo".into(),
            password: "bar".into(),
        };

        assert!(UserCredentials::can_authenticate(&not_empty));
    }

    #[test]
    fn check_new_auth_credentials() {
        let lhs = Credentials {
            api_key: "Key".into(),
            api_secret: "Secret".into(),
            credentials: None,
            session_key: None,
        };
        let rhs = Credentials::new_partial("Key", "Secret");

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn check_set_user_creds() {
        let mut auth_creds = Credentials::new_partial("Key".into(), "Secret".into());
        auth_creds.set_user_credentials("Username".into(), "Password".into());

        let internal_creds = auth_creds.credentials.unwrap();

        let creds = match internal_creds {
            CredentialsVariant::UserSupplied(val) => val,
            _ => panic!("Invalid UserCredentials Value"),
        };

        assert_eq!(creds.username, "Username");
        assert_eq!(creds.password, "Password");
    }

    #[test]
    fn check_set_user_token() {
        let mut auth_creds = Credentials::new_partial("Key".into(), "Secret".into());
        auth_creds.set_user_token("Token".into());

        let token = auth_creds.credentials.unwrap();

        let token = match token {
            CredentialsVariant::Token(val) => val,
            _ => panic!("Invalid Token"),
        };

        assert_eq!(token, "Token");
    }

    #[test]
    fn check_set_session_key_and_is_authed() {
        let mut auth_creds = Credentials::new_partial("Key".into(), "Secret".into());
        auth_creds.set_session_key("SomeKey".into());
        let key = auth_creds.session_key().unwrap();

        assert_eq!(key, "SomeKey");
        assert!(auth_creds.is_authenticated());
    }

    #[test]
    fn check_auth_req_params_and_get_signature() {
        let mut auth_creds = Credentials::new_partial("Key".into(), "Secret".into());
        auth_creds.set_user_token("Token".into());
        let param_map = auth_creds.get_auth_request_params().unwrap();

        assert_eq!(param_map["token"], "Token");

        auth_creds.set_user_credentials("Foo".into(), "Bar".into());
        let param_map = auth_creds.get_auth_request_params().unwrap();

        assert_eq!(param_map["username"], "Foo");
        assert_eq!(param_map["password"], "Bar");
    }

    #[test]
    #[should_panic]
    fn check_get_bad_params() {
        let auth_creds = Credentials::new_partial("Key", "Secret");
        auth_creds.get_auth_request_params().unwrap();
    }

    #[test]
    fn check_req_params() {
        let mut auth_creds = Credentials::new_partial("Key".into(), "Secret".into());
        auth_creds.set_session_key("SomeKey".into());
        let req_params = auth_creds.get_request_params();

        assert_eq!(req_params["api_key".into()], "Key");
        assert_eq!(req_params["sk".into()], "SomeKey");
    }
}
