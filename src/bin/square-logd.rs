use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use color_eyre::eyre::Result;
use square_log::{
    config::Config,
    db::sled::SledDatabase,
    http::axum::HttpServer,
    plugin::{registry::PluginRegistry, scheduler::Scheduler},
};
use tokio::task::LocalSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long = "config")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args { config } = Args::parse();
    let config = serde_yaml::from_reader::<_, Config>(std::fs::File::open(config)?)?;

    // Initialize database.
    let db = Arc::new(SledDatabase::new(config.db)?);

    // Load plugins and sources configs.
    let plugin_registry = PluginRegistry::new(config.plugins, db.clone())?;

    let mut scheduler = Scheduler::new(plugin_registry);

    let server = HttpServer::new(config.http, db);

    // To have async api functions exposed to lua side (lua.call_async), source runtimes need to be
    // executed in the same thread.
    let scheduler_task = LocalSet::new();
    scheduler_task.spawn_local(async move {
        scheduler.spawn().await;
    });

    // Start http api.
    let http_task = tokio::spawn(async move {
        let _ = server.serve().await;
    });

    tokio::select! {
        _ = scheduler_task => {
            eprintln!("Local task completed");
        }
        _ = http_task => {
            eprintln!("Regular task completed");
        }
    };

    Ok(())
}
