use std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod api;
pub mod registry;

// Functions expected to be available from lua site to rust.
const LUA_PLUGIN_CALL_FN: &str = "sqrt_call_fn";

// Functions available from rust to lua via RUST_API_GLOBAL_NAME.
const RUST_API_GLOBAL_NAME: &str = "sqrt_log";
const RUST_API_FETCH_JSON_FN: &str = "fetch_json";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PluginConfig {
    pub path: PathBuf,
}
