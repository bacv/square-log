use std::sync::Arc;

use mlua::{Lua, LuaSerdeExt, Result, UserData, UserDataMethods, Value};

use crate::{
    db::Database,
    plugin::{RUST_API_GET_LATEST_FN, RUST_API_INSERT_REC_FN},
    record::DataRecord,
};

#[derive(Clone)]
pub struct DbApi<DB> {
    db: Arc<DB>,
    source: String,
}

impl<DB: Database> DbApi<DB> {
    pub fn new(db: Arc<DB>, source: String) -> Self {
        Self { db, source }
    }

    fn insert(&self, data: DataRecord) -> Result<()> {
        let _ = self.db.insert(&self.source, data);
        Ok(())
    }

    fn latest<'lua>(&self, lua: &'lua Lua) -> Result<Value<'lua>> {
        let maybe_record = self
            .db
            .get_source(&self.source)
            .map_err(mlua::Error::external)
            .map(|maybe_summary| maybe_summary.and_then(|summary| summary.latest))?;

        match maybe_record {
            Some(record) => lua.to_value(&record),
            None => Ok(Value::Nil),
        }
    }
}

impl<DB: Database + 'static> UserData for DbApi<DB> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(RUST_API_INSERT_REC_FN, |_, api, data: DataRecord| {
            api.insert(data)
        });

        methods.add_method(RUST_API_GET_LATEST_FN, |lua, api, _: ()| api.latest(lua));
    }
}
