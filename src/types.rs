// src/model.rs
//! Defines data structure for crates.io response and final output 


use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Response shape for crates.io top-level list endpoint 
#[derive(Debug, Serialize)]
pub struct CratesResponse {
    pub crates : Vec<Crates>,
    pub meta : Meta,
}

/// Minimal crate records to store. Rest to keep in `extra` for extensibility
/// uses `Value` which is type enum for handling unknown fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crates {
    pub id: String,
    pub name: String,
    pub max_version: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    // `default` helps in handing missing data as `None`
    pub description: Option<String>,
    // Helps additional keys to be stored in extra
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}


/// Pagnination Meta data from crates.io 
pub struct Meta {
    pub total : u64,
    #[serde(default)]
    pub next_page : Option<u64>,
    #[serde(default)]
    pub previous_page : Option<u64>,
}


/// Final JSON for all crates using Ratatui
/// uses `Value` which is type enum for handling unknown fields
#[derive(Debug, Serialize)]
pub struct Ratcrate {
    pub crate_name: String,
    pub crate_id: String,
    pub max_version: Option<String>,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub cargo_toml_url: String,
    pub uses_ratatui: bool,
    pub extra: HashMap<String, Value>,
}
