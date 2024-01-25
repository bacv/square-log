use mlua::{FromLua, Lua, Result as LuaResult, Table, UserData, Value};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataRecord {
    pub id: Option<usize>,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: String,
    pub extended: String,
    pub hash: String,
    pub origin_timestamp: i64,
    pub pull_timestamp: i64,
}

impl UserData for DataRecord {}

impl<'lua> FromLua<'lua> for DataRecord {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let table: Table = match value {
            Value::Table(table) => table,
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "DataRecord",
                    message: Some("expected a table".to_string()),
                });
            }
        };

        Ok(DataRecord {
            id: None, // Overriden by the db.
            title: table.get("title").or_else(default_str("title"))?,
            description: table
                .get("description")
                .or_else(default_str("description"))?,
            tags: table.get("tags").unwrap_or_else(|_| Vec::new()),
            link: table.get("link").or_else(default_str("link"))?,
            extended: table.get("extended").or_else(default_str("extended"))?,
            hash: table.get("hash").or_else(default_str("hash"))?,
            origin_timestamp: table.get("origin_timestamp").unwrap_or(0),
            pull_timestamp: table.get("pull_timestamp").unwrap_or(0),
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
