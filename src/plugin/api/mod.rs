use std::sync::Arc;

use mlua::{UserData, UserDataFields};

use crate::db::Database;

mod db;
mod http;

use db::DbApi;
use http::HttpApi;

pub struct Api<DB> {
    db: Arc<DbApi<DB>>,
}

impl<DB: Database> Api<DB> {
    pub fn new(db: Arc<DB>, source: String) -> Self {
        Self {
            db: Arc::new(DbApi::new(db, source)),
        }
    }
}

impl<DB: Database + Send + Sync + 'static> UserData for Api<DB> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field("http", HttpApi);
        fields.add_field_method_get("db", |_, api| Ok(api.db.clone()));
    }
}
