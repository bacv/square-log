use std::{path::PathBuf, sync::Arc, time::Duration};

use clap::Parser;
use color_eyre::eyre::Result;
use square_log::{
    config::Config,
    db::{mock::MockDatabase, Database},
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
    let db = Arc::new(MockDatabase::new());

    // Load plugins and sources configs.
    let plugin_registry = PluginRegistry::new(config.plugins, db.clone())?;

    let mut scheduler = Scheduler::new(plugin_registry);

    // To have async api functions exposed to lua side (lua.call_async), source runtimes need to be
    // executed in the same thread.
    let local = LocalSet::new();
    local.spawn_local(async move {
        scheduler.spawn().await;
    });

    // Start http api.
    let http_task = tokio::spawn(async move {
        loop {
            let recs = db.get_latest("sample".into());
            println!("{recs:?}");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::select! {
        _ = local => {
            eprintln!("Local task completed");
        }
        _ = http_task => {
            eprintln!("Regular task completed");
        }
    };

    Ok(())
}
