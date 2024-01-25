use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub mod axum;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpConfig {
    addr: SocketAddr,
}
