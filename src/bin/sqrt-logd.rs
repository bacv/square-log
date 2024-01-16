use std::path::{Path, PathBuf};

use clap::Parser;
use color_eyre::eyre::Result;
use mlua::Lua;
use sqrt_log::config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long = "config")]
    config: PathBuf,
}

fn load_lua_scripts(lua: &Lua, config_path: &Path) -> mlua::Result<()> {
    for entry in std::fs::read_dir(config_path)? {
        let path = entry?.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("lua") {
            let script = std::fs::read_to_string(path)?;
            lua.load(&script).exec()?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let Args { config } = Args::parse();
    let config = serde_yaml::from_reader::<_, Config>(std::fs::File::open(config)?)?;

    Ok(())
}
