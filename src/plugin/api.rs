use mlua::{Function, Lua, Result};

pub fn fetch_json(lua: &Lua) -> Result<Function> {
    lua.create_async_function(|_, uri: String| async move {
        // let resp = reqwest::get(&uri)
        //     .await
        //     .and_then(|resp| resp.error_for_status())
        //     .into_lua_err()?;
        // let json = resp.json::<serde_json::Value>().await.into_lua_err()?;
        // lua.to_value(&json)
        println!("hello from rust: {uri}");
        Ok(())
    })
}
