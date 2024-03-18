use mlua::{chunk, Lua, Result};

fn main() -> Result<()> {
    let lua = Lua::new();

    lua.load(chunk! {
        local user_lib = require "user_lib"
        user_lib.hello()
    })
    .exec()
}