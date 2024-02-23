use std::{fs, path::Path, sync::Arc, time::Duration};

use mlua::{Function, Lua, LuaSerdeExt, Result, Table};

use crate::db::Database;

use super::{
    api::Api, source::Source, PluginConfig, LUA_SOURCES_FN, LUA_SOURCES_VAR, RUST_API_GLOBAL_NAME,
};

const DEFAULT_INTERVAL: Duration = Duration::from_secs(10);

pub struct PluginRegistry {
    pub sources: Vec<Source>,
}

impl PluginRegistry {
    pub fn new<DB>(config: PluginConfig, db: Arc<DB>) -> Result<Self>
    where
        DB: Database + Send + Sync + 'static,
    {
        // Lua runtime for loading sources, won't be used after initialization.
        let init_rt = Lua::new();
        let script = fs::read_to_string(config.sources)?;
        init_rt.load(script).exec()?;

        // Retrieve the global function defined in Lua.
        let sources_fn: Function = init_rt
            .globals()
            .get(LUA_SOURCES_FN)
            .map_err(|_| mlua::Error::RuntimeError("Failed to get global function".to_string()))?;

        let mut sources = Vec::new();
        let plugin_list: Table = sources_fn.call(())?;

        for plugin in plugin_list.pairs::<mlua::String, mlua::Table>() {
            let (plugin, source_list) = plugin?;
            for source in source_list.pairs::<mlua::Value, mlua::Table>() {
                let (_, source_table) = source?;
                let source_rt = Lua::new();
                let source = load_source(source_rt, source_table)?;

                let api = Api::new(db.clone(), source.id.clone());
                // TODO: Plugin script is reaad from file multiple times.
                load_plugin(&source.rt, plugin.to_str()?, &config.directory, api)?;

                sources.push(source);
            }
        }

        Ok(Self { sources })
    }
}

fn load_source(rt: Lua, source_table: Table) -> Result<Source> {
    // Parse mandatory fields of the source definition on lua side.
    let id: String = source_table
        .get("id")
        .map_err(|_| mlua::Error::RuntimeError("Failed to get 'url' from source".to_string()))?;
    let interval: Duration = source_table
        .get::<_, String>("interval")
        .map(|i| humantime::parse_duration(&i).unwrap_or(DEFAULT_INTERVAL))
        .map_err(|_| {
            mlua::Error::RuntimeError("Failed to get 'interval' from source".to_string())
        })?;
    // TODO: refactor so that there is no need for state copies between rt instances.
    let json = serde_json::to_value(source_table).expect("Should serialize to JSON");
    rt.globals().set(LUA_SOURCES_VAR, rt.to_value(&json)?)?;

    let source = Source::new(id, interval, rt);
    Ok(source)
}

fn load_plugin<DB>(rt: &Lua, name: &str, directory: &Path, api: Api<DB>) -> Result<()>
where
    DB: Database + Send + Sync + 'static,
{
    let plugin_path = directory.join(format!("{name}.lua"));
    let script = fs::read_to_string(plugin_path)?;

    rt.load(&script).exec()?;
    rt.globals().set(RUST_API_GLOBAL_NAME, api)?;

    Ok(())
}
