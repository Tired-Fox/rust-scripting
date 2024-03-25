mod plugin;
mod prettify;

use mlua::{Error as LuaError, Lua, String as LuaString, Table};
pub use plugin::Plugins;
pub use prettify::Prettify;

pub trait Import {
    fn module_name() -> &'static str;
    fn import(lua: &Lua) -> Result<Table, LuaError>;

    fn module(lua: &Lua) -> Result<Table, LuaError> {
        let module = lua.globals().get::<_, Table>(Self::module_name())?;
        match module.get_metatable() {
            Some(meta) => {
                if let Ok(name) = meta.get::<_, LuaString>("__metatable") {
                    if Self::module_name() == name.to_str()? {
                        return Ok(module);
                    }
                }
            }
            _ => {}
        }
        Err(LuaError::RuntimeError(format!(
            "Global module {} has been overriden and can no longer be accessed. This value is not meant to be overriden.",
            Self::module_name()
        )))
    }
}

pub trait Require {
    fn require<I: Import>(&self) -> Result<(), LuaError>;
}

impl Require for Lua {
    fn require<I: Import>(&self) -> Result<(), LuaError> {
        let table = I::import(self)?;
        // define a meta value that allows for identification if
        // the module is still the rust module that was set from the start
        // this can still be easily overriden but that helps catch accidental
        // overrides of global tables
        match table.get_metatable() {
            Some(meta) => meta.set("__metatable", I::module_name())?,
            None => {
                let meta = self.create_table()?;
                meta.set("__metatable", I::module_name())?;
                table.set_metatable(Some(meta));
            }
        }
        self.globals().set(I::module_name(), table)
    }
}
