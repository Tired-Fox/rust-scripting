use mlua::{Lua, Table};
use mlua::prelude::LuaError;

pub mod http;
pub mod modules;

pub trait Import {
    fn module_name() -> &'static str;
    fn import(lua: &'static Lua) -> Result<Table, LuaError>;
}

pub trait Require {
    fn require<I: Import>(&'static self) -> Result<(), LuaError>;
}

impl Require for Lua {
    fn require<I: Import>(&'static self) -> Result<(), LuaError> {
        let table = I::import(self)?;
        self.globals().set(I::module_name(), table)
    }
}