use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use color_eyre::eyre::eyre;
use mlua::{Function, Lua, Result};

use super::{api::Api, PluginConfig, LUA_PLUGIN_CALL_FN, RUST_API_GLOBAL_NAME};
use crate::source::Source;

pub struct PluginRegistry {
    plugins: HashMap<String, Lua>,
}

impl PluginRegistry {
    pub fn init(config: PluginConfig) -> Result<Self> {
        let api = Arc::new(Api);

        let mut plugins = HashMap::new();
        for entry in fs::read_dir(config.path)? {
            let (plugin_name, plugin_source) = load_plugin(entry?.path(), &api)?;
            plugins.insert(plugin_name, plugin_source);
        }

        Ok(Self { plugins })
    }

    pub async fn call(&self, source: &Source) -> color_eyre::Result<()> {
        let call_fn: Function = self
            .plugins
            .get(&source.plugin)
            .ok_or_else(|| eyre!("invalid plugin name"))?
            .globals()
            .get(LUA_PLUGIN_CALL_FN)?;

        call_fn.call_async::<_, ()>(source.address.clone()).await?;
        Ok(())
    }
}

fn load_plugin(path: PathBuf, api: &Arc<Api>) -> Result<(String, Lua)> {
    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("lua") {
        let lua = Lua::new();
        let script = fs::read_to_string(&path)?;
        let plugin_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| mlua::Error::RuntimeError("Invalid plugin file name".to_string()))?
            .to_string();

        lua.load(&script).exec()?;
        lua.globals().set(RUST_API_GLOBAL_NAME, api.clone())?;

        Ok((plugin_name, lua))
    } else {
        Err(mlua::Error::RuntimeError(
            "File is not a Lua script".to_string(),
        ))
    }
}
