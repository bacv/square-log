use std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod api;
pub mod registry;

const PLUGIN_CALL_FN: &str = "sqrt_call_fn";
const API_FETCH_JSON_FN: &str = "fetch_json";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PluginConfig {
    pub path: PathBuf,
}
