use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use sqrt_log::{
    config::Config,
    plugin::registry::PluginRegistry,
    source::{Source, Sources},
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

    // Read sources
    let mut sources = Sources::load(config.sources)?;
    sources.push(Source {
        address: "test".into(),
        plugin: "sample".into(),
    });

    // Load plugins
    let plugin_registry = PluginRegistry::init(config.plugins)?;
    plugin_registry.call(&sources[0]).await?;

    // Start the pull loop
    //let log_puller = LogPuller::init(sources, plugin_registry).start().await?;

    Ok(())
}
