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

#[derive(Deserialize, Debug)]
pub struct NowPlayingResponseWrapper {
    pub nowplaying: NowPlayingResponse
}

#[derive(Deserialize, Debug)]
pub struct NowPlayingResponse {
    pub artist: CorrectableString,
    pub album: CorrectableString,
    #[serde(rename="albumArtist")]
    pub album_artist: CorrectableString,
    pub track: CorrectableString
}

#[derive(Deserialize, Debug)]
pub struct CorrectableString {
    pub corrected: String,
    #[serde(rename="#text")]
    pub text: String
}
