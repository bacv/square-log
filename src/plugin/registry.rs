use color_eyre::eyre::eyre;
use mlua::{Function, Lua};
use std::{collections::HashMap, fs};

use crate::source::Source;

use super::{api, PluginConfig, API_FETCH_JSON_FN, PLUGIN_CALL_FN};

pub struct PluginRegistry {
    plugins: HashMap<String, Lua>,
}

impl PluginRegistry {
    pub fn init(config: PluginConfig) -> mlua::Result<Self> {
        let mut registry = Self {
            plugins: HashMap::new(),
        };

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
                lua.globals()
                    .set(API_FETCH_JSON_FN, api::fetch_json(&lua)?)?;

                registry.plugins.insert(plugin_name, lua);
            }
        }

        Ok(registry)
    }

    pub async fn call(&self, source: &Source) -> color_eyre::Result<()> {
        let call_fn: Function = self
            .plugins
            .get(&source.plugin)
            .ok_or_else(|| eyre!(""))?
            .globals()
            .get(PLUGIN_CALL_FN)?;

        call_fn.call_async::<_, ()>(source.address.clone()).await?;
        Ok(())
    }
}
