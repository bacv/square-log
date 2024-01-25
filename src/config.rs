use serde::{Deserialize, Serialize};

use crate::{db::DbConfig, http::HttpConfig, plugin::PluginConfig};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    pub plugins: PluginConfig,
    pub db: DbConfig,
    pub http: HttpConfig,
}
