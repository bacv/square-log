use std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod api;
pub mod registry;
pub mod sheduler;
mod source;

// Functions expected to be available from lua site to rust.
const LUA_PLUGIN_CALL_FN: &str = "sqrt_call_fn";
// Function tha returns a list of sources for plugins.
const LUA_SOURCES_FN: &str = "sqrt_sources_fn";
const LUA_SOURCES_VAR: &str = "sqrt_sources_var";

// Functions available from rust to lua via RUST_API_GLOBAL_NAME.
const RUST_API_GLOBAL_NAME: &str = "sqrt_log";
const RUST_API_FETCH_JSON_FN: &str = "fetch_json";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PluginConfig {
    pub directory: PathBuf,
    pub sources: PathBuf,
}
