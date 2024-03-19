use mlua::{Lua, Table};
use mlua::prelude::LuaError;
use crate::Import;

mod client;
mod server;

pub struct Http;

impl Import for Http {
    fn module_name() -> &'static str { "http" }
    fn import(lua: &'static Lua) -> Result<Table, LuaError> {
        let http = lua.create_table()?;
        http.set("Client", client::Client)?;
        http.set("Server", server::Server(lua))?;
        Ok(http)
    }
}