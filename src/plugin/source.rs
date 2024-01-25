use std::{
    ops::Add,
    time::{Duration, Instant},
};

use mlua::{Function, Lua, Value};

use super::{LUA_PLUGIN_CALL_FN, LUA_SOURCES_VAR};

pub struct Source {
    pub id: String,
    pub interval: Duration,
    pub next_run: Instant,
    pub rt: Lua,
}

impl Source {
    pub fn new(id: String, interval: Duration, rt: Lua) -> Self {
        Self {
            id,
            interval,
            rt,
            next_run: Instant::now().add(interval),
        }
    }

    pub fn should_run(&self) -> bool {
        Instant::now() >= self.next_run
    }

    pub fn update_next_run(&mut self) {
        self.next_run = Instant::now().add(self.interval);
    }

    pub async fn run(&mut self) {
        let lua_source: Value = self
            .rt
            .globals()
            .get(LUA_SOURCES_VAR)
            .expect("Source should be set in lua ");
        let lua_call_fn: Function = self
            .rt
            .globals()
            .get(LUA_PLUGIN_CALL_FN)
            .expect("Call function should be set in lua plugin");

        // TODO: handle lua panic;
        lua_call_fn
            .call_async::<_, ()>(lua_source)
            .await
            .expect("TODO: don't panic");
        drop(lua_call_fn);

        self.update_next_run();
    }
}
