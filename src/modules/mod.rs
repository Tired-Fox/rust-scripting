mod plugin;
mod prettify;

use mlua::{Error as LuaError, Lua, Table};
pub use plugin::Plugins;
pub use prettify::{Prettify, pformat};

pub trait Import {
    /// The name of the module
    fn module_name() -> &'static str;
    
    /// Extend an existing table with the modules contents
    fn extend(table: &Table, lua: &Lua) -> Result<(), LuaError>;

    /// Create the module and return it (Import)
    fn import(lua: &Lua) -> Result<Table, LuaError> {
        let table = lua.create_table()?;
        Self::extend(&table, lua)?;
        Ok(table)
    }
}

pub trait Require {
    fn require<I: Import>(&self) -> Result<(), LuaError>;
    fn import<S: AsRef<str>>(&self, name: S, module: Table<'_>) -> Result<(), LuaError>;
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

    fn import<S: AsRef<str>>(&self, name: S, module: Table<'_>) -> Result<(), LuaError> {
        self.globals().set(name.as_ref(), module)?;
        Ok(())
    }
}
