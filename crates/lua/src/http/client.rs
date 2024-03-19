use mlua::prelude::LuaString;
use mlua::UserData;

#[derive(Clone, Debug, Default)]
pub struct Client;

impl UserData for Client {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |_lua, _, url: LuaString| {
            println!("GET {}", url.to_str()?);
            Ok(())
        })
    }
}