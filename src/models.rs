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
        #[serde(rename = "albumArtist")]
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
    #[CollectionName = "ScrobbleList"]
    #[CollectionDerives = "Debug, Deserialize"]
    pub struct ScrobbleResponse {
        pub artist: CorrectableString,
        pub album: CorrectableString,
        #[serde(rename = "albumArtist")]
        pub album_artist: CorrectableString,
        pub track: CorrectableString,
        pub timestamp: String,
        //  TODO: Ignored field here? (#20)
    }

    #[derive(Debug)]
    pub struct BatchScrobbleResponse {
        pub scrobbles: ScrobbleList,
    }

    #[derive(Deserialize, Debug)]
    pub struct BatchScrobbleResponseWrapper {
        pub scrobbles: BatchScrobbles,
    }

    #[derive(Deserialize, Debug)]
    pub struct BatchScrobbles {
        #[serde(rename = "scrobble")]
        pub scrobbles: ScrobbleList,
    }

    #[derive(Deserialize, Debug)]
    pub struct CorrectableString {
        #[serde(deserialize_with = "CorrectableString::deserialize_corrected_field")]
        pub corrected: bool,
        #[serde(rename = "#text", default)]
        pub text: String,
    }

    impl CorrectableString {
        fn deserialize_corrected_field<'de, D>(de: D) -> Result<bool, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let deser_result: json::Value = serde::Deserialize::deserialize(de)?;
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
    #[CollectionName = "ScrobbleBatch"]
    #[CollectionDoc = "A batch of Scrobbles to be submitted to Last.fm together."]
    #[CollectionDerives = "Clone, Debug"]
    pub struct Scrobble {
        artist: String,
        track: String,
        album: String,

        timestamp: Option<u64>,
    }

    impl Scrobble {
        /// Constructs a new Scrobble instance, representing a music track
        /// played in the past.
        pub fn new(artist: &str, track: &str, album: &str) -> Self {
            Self {
                artist: artist.to_owned(),
                track: track.to_owned(),
                album: album.to_owned(),
                timestamp: None,
            }
        }

        /// Sets the timestamp (date/time of play) of a Scrobble. Used in a builder-style pattern, typically after 
        /// [`Scrobble::new`].
        ///  
        /// **Note:**  Scrobbles without timestamps are automatically assigned a timestamp of the current time when
        /// submitted via [`Scrobbler::scrobble`] or [`Scrobbler::scrobble_batch`]. Timestamps only need to be 
        /// explicitly set when you are submitting a Scrobble at a point in the past, or in the future.
        /// 
        /// # Example
        /// ```
        /// let mut scrobble = Scrobble::new(...).with_timestamp(12345)
        /// ```
        /// 
        /// [`Scrobble::new`]: struct.Scrobble.html#method.new
        /// [`Scrobbler::scrobble`]: struct.Scrobbler.html#method.scrobble
        /// [`Scrobbler::scrobble_batch`]: struct.Scrobbler.html#method.scrobble_batch
        pub fn with_timestamp(&mut self, timestamp: u64) -> &mut Self {
            self.timestamp = Some(timestamp);
            self
        }

        /// Converts the Scrobble metadata (track name, artist & album name) into a [`HashMap`]. Map keys are 
        /// `"track"`, `"artist"` and `"album"`. If a timestamp is set, it will be present in the map under key 
        /// `"timestamp"`.
        /// 
        /// # Example
        /// ```
        /// let scrobble = Scrobble::new("Example Artist", ...);
        /// let scrobble_map = scrobble.as_map();
        /// assert_eq!(scrobble_map.get("artist"), "Example Artist");
        /// ```
        /// 
        /// [`HashMap`]: struct.HashMap.html
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

        pub fn artist(&self) -> &str {
            &self.artist
        }

        pub fn track(&self) -> &str {
            &self.track
        }

        pub fn album(&self) -> &str {
            &self.album
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn make_scrobble() {
            let mut scrobble = Scrobble::new(
                "foo floyd and the fruit flies",
                "old bananas",
                "old bananas",
            );
            scrobble.with_timestamp(1337);
            assert_eq!(scrobble.artist(), "foo floyd and the fruit flies");
            assert_eq!(scrobble.track(), "old bananas");
            assert_eq!(scrobble.album(), "old bananas");
            assert_eq!(scrobble.timestamp, Some(1337));
        }

        #[test]
        fn make_scrobble_check_map() {
            let scrobble = Scrobble::new(
                "foo floyd and the fruit flies",
                "old bananas",
                "old bananas",
            );

            let params = scrobble.as_map();
            assert_eq!(params["artist"], "foo floyd and the fruit flies");
            assert_eq!(params["track"], "old bananas");
            assert_eq!(params["album"], "old bananas");
        }
    }
}
