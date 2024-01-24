use chrono::{NaiveDate, ParseError};
use mlua::{FromLua, Lua, Result as LuaResult, Table, UserData, Value};

#[derive(Debug, Clone)]
pub struct DataRecord {
    pub date: NaiveDate,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: String,
    pub extended: String,
    pub hash: String,
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
                })
            }
        };

        let date_str: String =
            table
                .get("date")
                .map_err(|_| mlua::Error::FromLuaConversionError {
                    from: "nil or not a string",
                    to: "String",
                    message: Some("field 'date' is missing or invalid".to_string()),
                })?;
        let date: NaiveDate =
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e: ParseError| {
                mlua::Error::FromLuaConversionError {
                    from: "String",
                    to: "NaiveDate",
                    message: Some(format!("error parsing date: {}", e)),
                }
            })?;

        Ok(DataRecord {
            date,
            title: table.get("title").or_else(default_str("title"))?,
            description: table
                .get("description")
                .or_else(default_str("description"))?,
            tags: table.get("tags").unwrap_or_else(|_| Vec::new()),
            link: table.get("link").or_else(default_str("link"))?,
            extended: table.get("extended").or_else(default_str("extended"))?,
            hash: table.get("hash").or_else(default_str("hash"))?,
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
