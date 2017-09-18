pub mod responses {

    use std::fmt;

    use serde;
    use serde_json as json;

    #[derive(Deserialize, Debug)]
    pub struct AuthResponse {
        pub session: SessionResponse,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct SessionResponse {
        pub key: String,
        pub subscriber: i64,
        pub name: String,
    }

    #[derive(Deserialize)]
    pub struct NowPlayingResponseWrapper {
        pub nowplaying: NowPlayingResponse,
    }

    #[derive(Deserialize, Debug)]
    pub struct NowPlayingResponse {
        pub artist: CorrectableString,
        pub album: CorrectableString,
        #[serde(rename="albumArtist")]
        pub album_artist: CorrectableString,
        pub track: CorrectableString,
    }

    #[derive(Deserialize)]
    pub struct ScrobbleResponseWrapper {
        pub scrobbles: Scrobbles,
    }

    #[derive(Deserialize)]
    pub struct Scrobbles {
        pub scrobble: ScrobbleResponse,
    }

    #[derive(Deserialize, Debug)]
    pub struct ScrobbleResponse {
        pub artist: CorrectableString,
        pub album: CorrectableString,
        #[serde(rename="albumArtist")]
        pub album_artist: CorrectableString,
        pub track: CorrectableString,
        pub timestamp: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct CorrectableString {
        #[serde(deserialize_with="CorrectableString::deserialize_corrected_field")]
        pub corrected: bool,
        #[serde(rename="#text", default)]
        pub text: String,
    }

    impl CorrectableString {
        fn deserialize_corrected_field<'de, D>(de: D) -> Result<bool, D::Error>
            where D: serde::Deserializer<'de>
        {
            let deser_result: json::Value = try!(serde::Deserialize::deserialize(de));
            match deser_result {
                json::Value::String(ref s) if &*s == "1" => Ok(true),
                json::Value::String(ref s) if &*s == "0" => Ok(false),
                _ => Err(serde::de::Error::custom("Unexpected value")),
            }
        }
    }

    impl fmt::Display for CorrectableString {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.text)
        }
    }

}

pub mod metadata {

    use std::collections::HashMap;

    pub struct Scrobble {
        artist: String,
        track: String,
    }

    impl Scrobble {

        pub fn new(artist: String, track: String) -> Scrobble {
            Scrobble{ artist: artist, track: track }
        }

        pub fn as_map(&self) -> HashMap<&str, String> {
            let mut params = HashMap::new();
            params.insert("track", self.track.clone());
            params.insert("artist", self.artist.clone());

            params
        }

    }

}