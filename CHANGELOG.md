Version 0.3.0 - 2017-09-26

  * New Scrobble struct representing a single track-play (issue #8)
    * Update Scrobbler methods to take Scrobble struct instead of artist & 
      track names as bare Strings.
  * Add support for submitting album data along with track name & artist (#7)


Version 0.2.2 - 2017-09-17
==========================

  * Add timestamp field to ScrobblerResponse (Fixes issue #4)
  * Update to stable Serde v1.0.2
     * Update custom deserializer for CorrectableString
  * Refactor dto.rs into models.rs with sub-packages


Version 0.2.1 - 2017-08-26
==========================

  * Upgrade reqwest dependency to version 0.7.3, fixes reqwest bug with 
    connection pools timing out which broke long-existing Scrobbler instances


Version 0.2.0 - 2017-03-04
==========================

  * Return API responses encoded as structs (Issue #3)
    All Scrobbler methods now return the Last.fm API response deserialized
    into a Rust struct for the type (see API documentation)
  * Add ScrobbleError (with std::err:Error), instead of just returning
    error messages as strings.
  * Refactor Scrobbler/client code:
    * Move request-making code out of Scrobbler and into client; Scrobbler 
      should be an extremely minimal high(est) level layer.
    * Improve internal representation of API methods/operations, using an enum
      instead of passing magic strings around.


Version 0.1.2 - 2017-02-25
==========================

 * Improve API request code:
   * Switch to reqwest instead of hyper for HTTP client
   * Share a HTTP client instance between API requests
   * API request code refactor / cleanup
 * Update serde library dependency to version 0.9


Version 0.1.1 - 2017-02-22
==========================

 * Add API documentation for rustfm-scrobble crate
 * Update data in Cargo.toml


Version 0.1.0 - 2017-02-22
==========================

* Initial release
