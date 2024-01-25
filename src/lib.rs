pub mod config;
pub mod db;
pub mod http;
pub mod plugin;
pub mod record;

use once_cell::sync::Lazy;
use reqwest::Client;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);
