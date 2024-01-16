use std::{net::SocketAddr, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    pub plugins: PluginsConfig,
    pub db: DbConfig,
    pub sources: SourcesConfig,
    pub http: HttpConfig,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PluginsConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DbConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SourcesConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct HttpConfig {
    pub addr: SocketAddr,
}
