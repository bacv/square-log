use std::sync::Arc;

use mlua::{ExternalResult, LuaSerdeExt, String as LuaString, UserData, UserDataMethods};

use super::{RUST_API_FETCH_JSON_FN, RUST_API_INSERT_REC_FN};
use crate::{db::Database, record::DataRecord, CLIENT};

pub struct Api<DB> {
    db: Arc<DB>,
}

impl<DB> Api<DB> {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

impl<DB: Database + 'static> UserData for Api<Arc<DB>> {
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

        methods.add_method(RUST_API_INSERT_REC_FN, |_, api, data: DataRecord| {
            // TODO: Return failure result back to lua.
            let _ = api.db.insert("sample".into(), data.hash.clone(), data);
            Ok(())
        });
    }
}
