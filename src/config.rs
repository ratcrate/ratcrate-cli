
//! Application constant and file paths

/// crates.io endpoint base (we build paginated URL from this)
/// &str are utf-8 and immutable
pub const CRATES_IO_BASE: &str = "https://crates.io/api/v1/crates";

///Github API base (used to build repo content endpoints)
pub const GITHUB_API_BASE: &str = "https://api.github.com";

/// path to ETag cache (maps URL -> etag String)
/// Etags are fingerprints stored on the webserver
/// Helps in caching by sending ETag in response and is stored locally
/// In the next request browser will send IF-None-Match header with ETag from previous response
/// If server returns 304, it means that nothing has changed. 200 on change
pub const ETAG_CACHE_PATH: &str = ".ratatui_cache/etags.json";

/// Path to intermediate list of crates (To avoid refecthing crates.io on every run)
pub const INTERMEDIATE_CACHE : &str = "crates.json";

/// Final output file with creates that uses ratatui
pub const FINAL_OUTPUT : &str = "ratcrate.json";
