use std::{collections::HashMap, fs, sync::Arc};

use color_eyre::eyre::eyre;
use mlua::{Function, Lua};

use super::{api::Api, PluginConfig, LUA_PLUGIN_CALL_FN, RUST_API_GLOBAL_NAME};
use crate::source::Source;

pub struct PluginRegistry {
    plugins: HashMap<String, Lua>,
}

impl PluginRegistry {
    pub fn init(config: PluginConfig) -> mlua::Result<Self> {
        let mut registry = Self {
            plugins: HashMap::new(),
        };

        let api = Arc::new(Api);

        for entry in fs::read_dir(config.path)? {
            let path = entry?.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("lua") {
                let lua = Lua::new();
                let script = fs::read_to_string(&path)?;
                let plugin_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| {
                        mlua::Error::RuntimeError("Invalid plugin file name".to_string())
                    })?
                    .to_string();

                lua.load(&script).exec()?;
                lua.globals().set(RUST_API_GLOBAL_NAME, api.clone())?;

                registry.plugins.insert(plugin_name, lua);
            }
        }

        Ok(registry)
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
