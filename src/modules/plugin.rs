use std::collections::HashMap;

use mlua::{FromLua, Function, IntoLua, Lua, LuaSerdeExt, Table, Value};
use mlua::prelude::{LuaError, LuaString};

use super::Import;

/// This object is only constructed from lua tables.
/// it is used for parsing/validating tables for plugins along
/// with collecting and using the data from lua. This object is
/// not meant to be stored long term inside of rust.
#[derive(Default)]
pub struct Plugin<'lua> {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,

    hooks: HashMap<String, Function<'lua>>,
}

impl<'lua> Plugin<'lua> {
    pub fn info(&self, lua: &'lua Lua) -> Result<Table<'lua>, LuaError> {
        let info = lua.create_table()?;
        info.set("name", self.name.clone())?;
        info.set("version", self.version.clone())?;
        info.set("author", self.author.clone())?;
        info.set("description", self.description.clone())?;
        Ok(info)
    }
}

impl<'lua> IntoLua<'lua> for Plugin<'lua> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let value = lua.create_table()?;
        value.set("name", self.name)?;
        value.set("version", self.version)?;
        value.set("author", self.author)?;
        value.set("description", self.description)?;

        for (k, v) in self.hooks {
            value.set(k, v)?;
        }

        value.into_lua(lua)
    }
}

impl<'lua> FromLua<'lua> for Plugin<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let value = Table::from_lua(value, lua)?;

        let mut hooks = HashMap::new();
        for pair in value.clone().pairs::<LuaString, Value>() {
            let (k, v) = pair?;
            if let Ok(v) = Function::from_lua(v, lua) {
                hooks.insert(k.to_str()?.to_string(), v);
            }
        }

        Ok(Self {
            name: lua.from_value::<String>(value.get("name")?)?,
            version: lua.from_value::<String>(value.get("version")?)?,
            author: lua.from_value::<String>(value.get("author")?)?,
            description: lua.from_value::<String>(value.get("description")?)?,
            hooks,
        })
    }
}

fn new_plugin(lua: &Lua, data: Value) -> Result<(), LuaError> {
    // Parse input to new plugin as a table mapping to `Plugin`
    //  This step is purely for validation purposes
    let plugin = Plugin::from_lua(data.clone(), lua)?;

    // Call any hooks for setup
    log::info!("[LUA] Adding plugin {}", plugin.name);
    if let Some(setup) = plugin.hooks.get("setup") {
        setup.call(plugin.info(lua)?)?;
    }

    let plugins = Plugins::module(lua)?.get::<_, Table>("plugins")?;
    plugins.set(plugins.raw_len() + 1, plugin)?;

    Ok(())
}

pub struct Plugins;

impl Plugins {
    pub fn get_plugins(lua: &Lua) -> Result<Vec<Plugin>, LuaError> {
        Vec::<Plugin>::from_lua(Plugins::module(lua)?.get("plugins")?, lua)
    }
}

impl Import for Plugins {
    fn module_name() -> &'static str {
        "plugins"
    }

    fn import(lua: &Lua) -> Result<Table, LuaError> {
        let table = lua.create_table()?;

        table.set("plugins", lua.create_table()?)?;
        table.set("new_plugin", lua.create_function(new_plugin)?)?;

        Ok(table)
    }
}