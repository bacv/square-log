use std::{collections::HashMap, fs, path::Path, sync::Arc, time::Duration};

use mlua::{Function, Lua, LuaSerdeExt, Result, Table};

use super::{api::Api, PluginConfig, LUA_SOURCES_FN, LUA_SOURCES_VAR, RUST_API_GLOBAL_NAME};

const DEFAULT_INTERVAL: Duration = Duration::from_secs(10);

pub struct Source {
    url: String,
    interval: Duration,
    rt: Lua,
}

pub struct PluginRegistry {
    sources: HashMap<String, Source>,
}

impl PluginRegistry {
    pub fn new(config: PluginConfig) -> Result<Self> {
        let api = Arc::new(Api);
        let rt = Lua::new();
        let script = fs::read_to_string(config.sources)?;
        rt.load(script).exec()?;

        // Retrieve the global function defined in Lua.
        let sources_fn: Function = rt
            .globals()
            .get(LUA_SOURCES_FN)
            .map_err(|_| mlua::Error::RuntimeError("Failed to get global function".to_string()))?;

        let mut sources = HashMap::new();
        let plugin_list: Table = sources_fn.call(())?;

        for plugin in plugin_list.pairs::<mlua::String, mlua::Table>() {
            let (plugin, source_list) = plugin?;
            for source in source_list.pairs::<mlua::Value, mlua::Table>() {
                let (_, source_table) = source?;
                let source_rt = Lua::new();
                load_plugin(&source_rt, plugin.to_str()?, &config.directory, &api)?;
                let source = load_source(source_rt, source_table)?;
                sources.insert(source.url.clone(), source);
            }
        }

        Ok(Self { sources })
    }
}

fn load_source(rt: Lua, source_table: Table) -> Result<Source> {
    // Parse mandatory fields of the source definition on lua side.
    let url: String = source_table
        .get("url")
        .map_err(|_| mlua::Error::RuntimeError("Failed to get 'url' from source".to_string()))?;
    let interval: Duration = source_table
        .get::<_, String>("interval")
        .map(|i| humantime::parse_duration(&i).unwrap_or(DEFAULT_INTERVAL))
        .map_err(|_| {
            mlua::Error::RuntimeError("Failed to get 'interval' from source".to_string())
        })?;

    // TODO: refactor so that there is no need for state copies between rt instances.
    let json_str = serde_json::to_string(&source_table).expect("Should serialize to json");
    rt.globals().set(LUA_SOURCES_VAR, rt.to_value(&json_str)?)?;

    let source = Source { url, interval, rt };
    Ok(source)
}

fn load_plugin(rt: &Lua, name: &str, directory: &Path, api: &Arc<Api>) -> Result<()> {
    let plugin_path = directory.join(format!("{name}.lua"));
    let script = fs::read_to_string(plugin_path)?;

    rt.load(&script).exec()?;
    rt.globals().set(RUST_API_GLOBAL_NAME, api.clone())?;

    Ok(())
}
