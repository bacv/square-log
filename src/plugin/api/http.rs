use mlua::{
    ExternalResult, Lua, LuaSerdeExt, Result, String as LuaString, Table, UserData,
    UserDataMethods, Value,
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::{plugin::RUST_API_FETCH_JSON_FN, CLIENT};

#[derive(Clone)]
pub struct HttpApi;

impl UserData for HttpApi {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(RUST_API_FETCH_JSON_FN, fetch_json);
    }
}

async fn fetch_json<'lua>(
    lua: &'lua Lua,
    _http: &HttpApi,
    (uri, headers): (LuaString<'lua>, Option<Table<'lua>>),
) -> Result<Value<'lua>> {
    let req_headers = lua_headers(headers).into_lua_err()?;

    let resp = CLIENT
        .get(uri.to_str()?)
        .headers(req_headers)
        .send()
        .await
        .and_then(|resp| resp.error_for_status())
        .map_err(mlua::Error::external)?;
    let json = resp
        .json::<serde_json::Value>()
        .await
        .map_err(mlua::Error::external)?;
    lua.to_value(&json)
}

fn lua_headers(headers: Option<Table<'_>>) -> Result<HeaderMap> {
    let mut req_headers = HeaderMap::new();

    if let Some(tbl) = headers {
        for pair in tbl.pairs::<String, String>() {
            let (key, value) = pair.into_lua_err()?;
            req_headers.insert(
                HeaderName::from_bytes(key.as_bytes()).into_lua_err()?,
                HeaderValue::from_str(&value).into_lua_err()?,
            );
        }
    }

    Ok(req_headers)
}
