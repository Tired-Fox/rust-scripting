extern crate slua;

use mlua::Lua;
use slua::{
    modules::{Plugins, Prettify, config::Config},
    prelude::*, LuaExt,
    lua as _lua
};

fn main() -> color_eyre::Result<()> {
    env_logger::init();

    let mut lua = Lua::new();

    lua.set_paths(&[
        "D:/Repo/Rust/scripting/lua/?.lua",
        "D:/Repo/Rust/scripting/lua/?/init.lua",
    ]);

    lua.require::<Plugins>()?;
    lua.import("v", _lua::module! { [lua]
        "print" => Prettify::pprint,
    })?;

    let _ = _lua::array! { [lua]
        Config::default(),
    };

    lua.globals().set("config", Config::default())?;

    // Load init.lua file. The init file and all requires should be using provided functions
    // to load and manipulate lua state. Then the rust side will read that state and execute
    // actions based the state.
    log::info!("[\x1b[31mRUST\x1b[39m] Loading init.lua");
    lua.load("require 'init'").eval()?;

    _lua::print!(
        lua.globals().get::<_, Config>("config").unwrap(),
    );

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
