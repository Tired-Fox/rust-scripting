use mlua::{Lua, Table};
use mlua::prelude::LuaError;

pub mod plugin;
pub mod modules;

pub trait Import {
    fn module_name() -> &'static str;
    fn import(lua: &'static Lua) -> Result<Table, LuaError>;

    fn module(lua: &Lua) -> Result<Table, LuaError> {
        lua.globals().get(Self::module_name())
    }
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