rustfm-scrobble
===============

[![Latest Version](https://img.shields.io/crates/v/rustfm-scrobble.svg)](https://crates.io/crates/rustfm-scrobble)
[![Rust](https://github.com/bobbo/rustfm-scrobble/workflows/Rust/badge.svg?branch=master&event=push)](https://github.com/bobbo/rustfm-scrobble/actions)

*rustfm-scrobble* is a [Last.fm Scrobble API 2.0](http://www.last.fm/api/scrobbling) crate for Rust. It allows easy 
acccess to the "scrobble" and "now playing" notification endpoints through a simple Rust API. It can be used to record
song-plays from music players, build analog scrobbling tools similar to [VinylScrobbler](https://vinylscrobbler.com/)
or work with IoT Devices. It was initially built to implement a 
[Spotify scrobbling service](https://github.com/bobbo/spotify-connect-scrobbler) using the 
[Spotify Connect Protocol](https://www.spotify.com/uk/connect/) when the 
[Alexa Spotify client](https://www.spotify.com/uk/amazonalexa/) did not support scrobbling plays to Last.fm.

## Features

* Scrobble songs to Last.fm ('scrobble' API endpoint)
* Publish now-playing song to Last.fm ('now playing' API endpoint)
* Batch scrobble support in `Scrobbler::scrobble_batch` and `ScrobbleBatch`
* Multiple authentication flows to gain permissions to publish to Last.fm user profile
    * Store a pre-authenticated session key & throw away secret data after initial authentication
* Simple error handling; each API operation returns a `Result` with a simple `Error` type on failure
* Unit tested

## Install
Add *rustfm-scrobble* to your **Cargo.toml**.

 ```
 [dependencies]
 rustfm-scrobble="1.1"
 ```

## Usage

* [API Documentation](https://docs.rs/rustfm-scrobble)
* [Code Examples](https://github.com/bobbo/rustfm-scrobble/tree/master/examples)
    * Example now-playing & scrobbling client
    * Example batch scrobbling client
    * `cargo build --examples`
    * `./target/debug/examples/example`
* Build: `cargo build`
* Run Unit tests: `cargo test`

```rust
extern crate rustfm_scrobble;
use rustfm_scrobble::{Scrobble, Scrobbler};

let username = "last-fm-username";
let password = "last-fm-password";
let api_key = "client-api-key";
let api_secret = "client-api-secret";
 
let mut scrobbler = Scrobbler::new(api_key, api_secret);
scrobbler.authenticate_with_password(username, password);
 
let song = Scrobble::new("Example Artist", "Example Song", "Example Album");
scrobbler.scrobble(song);
```

### In Use

*rustfm-scrobble* is used in several projects including [polaris](https://github.com/agersant/polaris), [connectr](https://github.com/mrmekon/connectr),
[rescrobbled](https://github.com/InputUsername/rescrobbled) and [rb-scrobbler](https://github.com/Jeselnik/rb-scrobbler).


## Status

The API is stable & backwards compatibility will be guaranteed for all 1.0 releases.


## License

MIT license, see `./LICENSE`.
