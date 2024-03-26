extern crate slua;

use mlua::Lua;
use slua::{
    modules::{Plugins, Prettify},
    prelude::*,
};

fn main() -> color_eyre::Result<()> {
    env_logger::init();

    let lua = Lua::new();

    lua.require::<Plugins>()?;
    lua.require::<Prettify>()?;

    // Load init.lua file. The init file and all requires should be using provided functions
    // to load and manipulate lua state. Then the rust side will read that state and execute
    // actions based the state.
    lua.load("require 'init'").eval()?;

    log::info!("[\x1b[31mRUST\x1b[39m] Loading plugins");
    let plugins = Plugins::get_plugins(&lua)?;
    for plugin in plugins.iter() {
        println!(
            "{} {} by {}\n  {}",
            plugin.name, plugin.version, plugin.author, plugin.description
        );
    }

    Ok(())
}
