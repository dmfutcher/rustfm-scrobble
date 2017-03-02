#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub session: SessionResponse
}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionResponse {
    pub key: String,
    pub subscriber: i64,
    pub name: String
}
