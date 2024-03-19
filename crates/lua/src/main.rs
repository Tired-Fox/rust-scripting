extern crate slua;

use slua::{Require, Import, modules::Http};
use mlua::{chunk, Lua};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let lua: &'static Lua = Box::leak(Box::new(Lua::new()));

    lua.require::<Http>()?;

    lua
        .load(chunk! {
            local main = require "main"
            main()
        })
        .eval()?;

    Ok(())
}
