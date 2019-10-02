pub mod responses {

    use std::fmt;

    use serde::Deserialize;
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
        pub scrobbles: SingleScrobble,
    }

    #[derive(Deserialize)]
    pub struct SingleScrobble {
        pub scrobble: ScrobbleResponse,
    }

    #[derive(Deserialize, Debug, WrappedVec)]
    #[CollectionName="ScrobbleList"]
    #[CollectionDerives="Debug, Deserialize"]
    pub struct ScrobbleResponse {
        pub artist: CorrectableString,
        pub album: CorrectableString,
        #[serde(rename="albumArtist")]
        pub album_artist: CorrectableString,
        pub track: CorrectableString,
        pub timestamp: String,
        //  TODO: Ignored field here? (#20)
    }

    #[derive(Debug)]
    pub struct BatchScrobbleResponse {
        pub scrobbles: ScrobbleList
    }

    #[derive(Deserialize, Debug)]
    pub struct BatchScrobbleResponseWrapper {
        pub scrobbles: BatchScrobbles,
    }

    #[derive(Deserialize, Debug)]
    pub struct BatchScrobbles {
        #[serde(rename="scrobble")]
        pub scrobbles: ScrobbleList
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
            let deser_result: json::Value = r#try!(serde::Deserialize::deserialize(de));
            match deser_result {
                json::Value::String(ref s) if &*s == "1" => Ok(true),
                json::Value::String(ref s) if &*s == "0" => Ok(false),
                _ => Err(serde::de::Error::custom("Unexpected value")),
            }
        }
    }

    impl fmt::Display for CorrectableString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.text)
        }
    }

}

pub mod metadata {

    use std::collections::HashMap;

    /// Repesents a single track play (aka a "scrobble")
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, WrappedVec)]
    #[CollectionName="ScrobbleBatch"]
    #[CollectionDoc="A batch of Scrobbles to be submitted to Last.fm together."]
    #[CollectionDerives="Clone, Debug"]
    pub struct Scrobble {
        artist: String,
        track: String,
        album: String,

        timestamp: Option<u64>
    }

    impl Scrobble {

        /// Constructs a new Scrobble instance, representing a music track
        /// played in the past.
        pub fn new(artist: String, track: String, album: String) -> Scrobble {
            Scrobble{ artist, track, album, timestamp: None }
        }

        pub fn with_timestamp(&mut self, timestamp: u64) -> &mut Scrobble {
            self.timestamp = Some(timestamp);
            self
        }

        /// Converts the Scrobble metadata (track name, artist & album name)
        /// into a HashMap. Map keys are `"track"`, `"artist"` and `"album"`,
        /// respectively.
        pub fn as_map(&self) -> HashMap<String, String> {
            let mut params = HashMap::new();
            params.insert("track".to_string(), self.track.clone());
            params.insert("artist".to_string(), self.artist.clone());
            params.insert("album".to_string(), self.album.clone());

            if let Some(timestamp) = self.timestamp {
                params.insert("timestamp".to_string(), timestamp.to_string());
            }

            params
        }

        pub fn artist(&self) -> String {
            self.artist.clone()
        }

        pub fn track(&self) -> String {
            self.track.clone()
        }

        pub fn album(&self) -> String {
            self.album.clone()
        }

    }

}
