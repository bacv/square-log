use std::{net::SocketAddr, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::plugin::PluginConfig;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    pub plugins: PluginConfig,
    pub db: DbConfig,
    pub http: HttpConfig,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DbConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct HttpConfig {
    pub addr: SocketAddr,
}
