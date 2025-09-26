
//! Etag caching and resilient HTTP fetching utilities
//!
//! This module stores a JSON map at `.ratatui_cache/etags.json` mapping URL -> ETag header
//! `fetch_with_etag` sends `if-None-Match` when available and updates the cache when a new ETag is returned
//!
//! `request_with_retry` wraps reqwest with exponential backoff + jitter fir transient failures
//!

use crate::config::ETAG_CACHE_PATH;
// shorthand for Result<(), Box <dyn std::error::Error>>
use anyhow::Result;
// import predefined const for HTTP header
use reqwest::header::{ETAG, IF_NONE_MATCH};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, time::Duration};
// async sleep
use tokio::time::sleep;
// logging framework for debugging and monitoring
use tracing::{debug, warn};


/// To hold cache data
#[derive(Debug, Serialize, Deserlialize)]
struct ETagCache {
    // keys are URL and values are ETag
    // hadhmap will help in fast etag lookup for given URL
    etags: HashMap<String, String>,
}

// ETagCache struct implementation
impl ETagCache {
    // reads ETag from path 
    fn load() -> Self {
        fs::read_to_string(ETAG_CACHE_PATH)
            // convert Result<T,E> into an Option<T>
            // we dont want the error case but want program to just continue
            .ok()
            // if successful, deserialize JSON to ETagCache struct
            .and_then(|s| serde_json::from str(&s).ok())
            // if deserialization fails, create an empty hashmp instead of panick
            .unwrap_or(ETagCache {
                etags: HashMap::new(),
            })
    }   

    //  Saves current state of cache in JSON 
    fn save(&self) -> Result<()> {
        // if parent directory doesn't exist, create it
        if let Some(parent) = Parent::new(ETAG_CACHE_PATH).parent() {
            std::fs::create_dir_all(parent);
        }
        // write to disk on the location mentioned by ETAG_CACHE_PATH
        fs::write(ETAG_CACHE_PATH, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    fn get(&self, url:&str) -> Option<String> {
        self.etags.get(url)
    }

    fn set(&mut self, url:&str, etag:&str) {
        self.etags.insert(url.to_string(), etag.to_string());
    }

}

// Result return from `fetch_eith_etag`
pub enum FetchResult {
    /// Server returned 304 Not Modified (body unchanged)
    NotModified,
    /// Server returned 200 OK with new body (modified or 1st time fetch)
    Modified(String),
}

/// Clone request retry wrapper: retries transient error and 5xx response
/// Using simple GET request without any streaming bodies so try_clone() is okay
