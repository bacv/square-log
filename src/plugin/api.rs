use mlua::{ExternalResult, LuaSerdeExt, String as LuaString, UserData, UserDataMethods};

use super::RUST_API_FETCH_JSON_FN;
use crate::CLIENT;

pub struct Api;

impl UserData for Api {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            RUST_API_FETCH_JSON_FN,
            |lua, _, uri: LuaString| async move {
                let resp = CLIENT
                    .get(uri.to_str()?)
                    .send()
                    .await
                    .and_then(|resp| resp.error_for_status())
                    .into_lua_err()?;
                let json = resp.json::<serde_json::Value>().await.into_lua_err()?;
                lua.to_value(&json)
            },
        );
    }
}
