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

    /// Repesents a single track play (aka a "scrobble")
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
            Scrobble{ artist: artist, track: track, album: album, timestamp: None }
        }

        pub fn with_timestamp(&mut self, timestamp: u64) -> &mut Scrobble {
            self.timestamp = Some(timestamp);
            self
        }

        /// Converts the Scrobble metadata (track name, artist & album name)
        /// into a HashMap. Map keys are `"track"`, `"artist"` and `"album"`,
        /// respectively.
        pub fn as_map(&self) -> HashMap<&str, String> {
            let mut params = HashMap::new();
            params.insert("track", self.track.clone());
            params.insert("artist", self.artist.clone());
            params.insert("album", self.album.clone());

            if let Some(timestamp) = self.timestamp {
                params.insert("timestamp", timestamp.to_string());
            }

            params
        }

    }

    /// Represents a collection of Scrobbles to be submitted in a single batch
    pub struct ScrobbleBatch(Vec<Scrobble>);

    type Iter<'a> = ::std::slice::Iter<'a, Scrobble>;
    type IntoIter = ::std::vec::IntoIter<Scrobble>;

    impl ::std::iter::FromIterator<Scrobble> for ScrobbleBatch {
        fn from_iter<I: IntoIterator<Item=Scrobble>>(iter: I) -> Self {
            let mut inner = vec![];
            inner.extend(iter);
            ScrobbleBatch(inner)
        }
    }

    impl From<Vec<Scrobble>> for ScrobbleBatch {

        fn from(ids: Vec<Scrobble>) -> ScrobbleBatch {
            let mut multipart = ScrobbleBatch::new();
            multipart.extend(ids);
            multipart
        }

    }

    impl IntoIterator for ScrobbleBatch {
        type Item = Scrobble;
        type IntoIter = IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    impl<'a> IntoIterator for &'a ScrobbleBatch {
        type Item = &'a Scrobble;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }

    impl Extend<Scrobble> for ScrobbleBatch {
        fn extend<T: IntoIterator<Item=Scrobble>>(&mut self, iter: T) {
            self.0.extend(iter);
        }
    }

    impl ScrobbleBatch {

        pub fn new() -> ScrobbleBatch {
            ScrobbleBatch(vec![])
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn iter<'a>(&'a self) -> Iter<'a> {
            self.into_iter()
        }

    }

}