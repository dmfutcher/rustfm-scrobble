rustfm-scrobble
===============

[![Latest Version](https://img.shields.io/crates/v/rustfm-scrobble.svg)](https://crates.io/crates/rustfm-scrobble)

*rustfm-scrobble* is a [Last.fm Scrobble API 2.0](http://www.last.fm/api/scrobbling)
library for Rust. It allows easy acccess to the "scrobble" and "now playing" notification
endpoints through a simple Rust API.

## Usage

* [API Documentation](https://docs.rs/rustfm-scrobble)
* [Examples](https://github.com/bobbo/rustfm-scrobble/tree/master/examples)

*rustfm-scrobble* exposes a single struct: `Scrobbler`. Use `Scrobbler::new()`
with your [API key and API secret](https://www.last.fm/api/account/create) to build a new `Scrobbler`. Call
`authenticate()` on your `Scrobbler` with the username & password of the user to
record scrobbles against (this matches the UX of most popular clients like Spotify).
Once the `Scrobbler` is authenticated, call `now_playing()` and `scrobble()` to
update the user's now playing track or log a new scrobbled track. Note that *rustfm-scrobble*
_does nothing to_ enforce [Last.fm's scrobble rules ](http://www.last.fm/api/scrobbling#when-is-a-scrobble-a-scrobble), this logic must
be implemented by the client program.

## Status
*rustfm-scrobble* is _beta_ quality. It is feature complete (authentication,
now playing and scrobbles all work correctly), however remains a work in progress
and some public API modifications are expected.

## License

MIT license, see `./LICENSE`.
