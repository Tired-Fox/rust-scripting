extern crate slua;

use mlua::{Lua, Value, Variadic};
use mlua::prelude::LuaError;

use slua::plugin::Plugins;
use slua::Require;

fn pformat(arg: &Value, indent: usize) -> Result<String, LuaError> {
    let spacing = " ".repeat(indent);
    Ok(match arg {
        Value::Nil => "nil".to_string(),
        Value::Boolean(bool) => format!("{}", bool),
        Value::LightUserData(_) => "Pointer".into(),
        Value::Integer(i) => format!("{:?}", i),
        Value::Number(n) => format!("{:?}", n),
        Value::String(s) => format!("{:?}", s.to_str().unwrap_or("")),
        Value::Table(t) => {
            let mut items: Vec<String> = Vec::new();
            for item in t.clone().pairs::<Value, Value>() {
                let (k, v) = item?;
                items.push(format!("{spacing}{} = {}", pformat(&k, indent+2)?, pformat(&v, indent+2)?));
            }

            format!("{{\n  {}\n{spacing}}}", items.join(",\n  "))
        }
        Value::Function(f) => {
            let info = f.info();
            format!(
                "Function({}line: {})",
                info.name.as_ref().map(|v| format!("{}, ", v)).unwrap_or(String::new()),
                info.line_defined.map(|v| v.to_string()).unwrap_or("???".into())
            )
        },
        Value::Thread(_) => "Thread".into(),
        Value::UserData(_) => "Any".into(),
        _ => format!("{:?}", arg),
    })
}

fn pprint(_: &Lua, args: Variadic<Value>) -> mlua::Result<()> {
    for arg in args.iter() {
        println!("{}", pformat(arg, 0)?);
    }
    Ok(())
}

macro_rules! set_global {
    ($lua: ident, function $name: ident ($($arg: ident: $atype: ty)* ) { $($body: tt)* }) => {
        $lua.globals().set(stringify!($name), $lua.create_function(|lua: &mlua::Lua, ( $($arg)* ): ( $($atype)* )| { $($body)* })?)?;
    };
    ($lua: ident, function $name: ident) => {
        $lua.globals().set(stringify!($name), $lua.create_function($name)?)?;
    };
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let lua: &'static Lua = Box::leak(Box::new(Lua::new()));

    lua.require::<Plugins>()?;
    set_global!(lua, function pprint);

    // Load init.lua file. The init file and all requires should be using provided functions
    // to load and manipulate lua state. Then the rust side will read that state and execute
    // actions based the state.
    lua.load("require 'init'").eval()?;

    println!("\nLoaded plugins:");
    let plugins = Plugins::get_plugins(lua)?;
    for plugin in plugins.iter() {
        println!("{} {} by {}\n  {}", plugin.name, plugin.version, plugin.author, plugin.description);
    }

    Ok(())
}
