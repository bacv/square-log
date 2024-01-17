pub mod config;
pub mod plugin;
pub mod record;
pub mod source;

use once_cell::sync::Lazy;
use reqwest::Client;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);
