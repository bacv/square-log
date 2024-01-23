use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use sqrt_log::{
    config::Config,
    plugin::{registry::PluginRegistry, scheduler::Scheduler},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long = "config")]
    config: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let Args { config } = Args::parse();
    let config = serde_yaml::from_reader::<_, Config>(std::fs::File::open(config)?)?;

    // initialize sled database

    // Load plugins and sources configs. Maybe pass sled db handle?
    let plugin_registry = PluginRegistry::new(config.plugins)?;

    let mut scheduler = Scheduler::new(plugin_registry.sources);

    // Start pulling
    scheduler.spawn().await;

    // Start http api

    // Wait for everything to finish

    Ok(())
}
