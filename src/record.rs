use std::collections::HashMap;

use mlua::{FromLua, Lua, Result as LuaResult, Table, UserData, Value};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataRecord {
    pub id: Option<usize>,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: String,
    pub extended: HashMap<String, String>,
    pub hash: String,
    pub origin_timestamp: i64,
    pub pull_timestamp: i64,
}

impl UserData for DataRecord {}

impl<'lua> FromLua<'lua> for DataRecord {
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        let lua_record: Table = match value {
            Value::Table(lua_record) => lua_record,
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "DataRecord",
                    message: Some("expected a table".to_string()),
                })
            }
        };

        let extended_table: Table = lua_record
            .get("extended")
            .or_else(default_table("extended"))?;

        let mut extended = HashMap::new();
        for pair in extended_table.pairs::<String, String>() {
            let (key, value) = pair?;
            extended.insert(key, value);
        }

        Ok(DataRecord {
            id: None, // Overridden by the db.
            title: lua_record.get("title").or_else(default_str("title"))?,
            description: lua_record
                .get("description")
                .or_else(default_str("description"))?,
            tags: lua_record.get("tags").unwrap_or_else(|_| Vec::new()),
            link: lua_record.get("link").or_else(default_str("link"))?,
            extended,
            hash: lua_record.get("hash").or_else(default_str("hash"))?,
            origin_timestamp: lua_record.get("origin_timestamp").unwrap_or(0),
            pull_timestamp: lua_record.get("pull_timestamp").unwrap_or(0),
        })
    }
}

fn default_table<'lua>(
    field: &'static str,
) -> impl Fn(mlua::Error) -> LuaResult<Table<'lua>> + 'lua {
    move |_| {
        Err(mlua::Error::FromLuaConversionError {
            from: "nil or not a table",
            to: "Table",
            message: Some(format!("field '{}' is missing or invalid", field)),
        })
    }
}

fn default_str(field: &str) -> impl Fn(mlua::Error) -> LuaResult<String> + '_ {
    move |_| {
        Err(mlua::Error::FromLuaConversionError {
            from: "nil or not a string",
            to: "String",
            message: Some(format!("field '{}' is missing or invalid", field)),
        })
    }
}
