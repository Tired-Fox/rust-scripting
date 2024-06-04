use mlua::FromLuaMulti;
use mlua::Function;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::Table;
use mlua::Error as LuaError;

use crate::modules::pformat;
use crate::modules::Import;

pub use crate::lua_print as print;
pub use crate::lua_module as module;


pub trait IntoLuaEntry<'lua, R, L = ()> {
    fn into_lua_entry(self, lua: &'lua Lua) -> Result<R, mlua::Error>;
}

impl<'lua, A, R, F> IntoLuaEntry<'lua, Function<'lua>, (A, R)> for F
where
    A: FromLuaMulti<'lua>,
    R: IntoLuaMulti<'lua>,
    F: Fn(&'lua Lua, A) -> Result<R, mlua::Error> + Send + 'static,
{
    fn into_lua_entry(self, lua: &'lua Lua) -> Result<Function<'lua>, mlua::Error> {
       lua.create_function(self) 
    }
}

impl<'lua, I: Import> IntoLuaEntry<'lua, Table<'lua>, ()> for I {
    fn into_lua_entry(self, _lua: &'lua Lua) -> Result<Table<'lua>, mlua::Error> {
        I::import(_lua)
    }
}

#[macro_export]
macro_rules! lua_module {
    {
        [$lua: ident] $($name: expr => $value: expr),* $(,)?
    } => {
        {
            use $crate::lua::IntoLuaEntry;
            let _m = $lua.create_table()?;
            $($crate::lua_module!{@-- ($lua, _m), $name: $value};)*
            Ok::<mlua::Table<'_>, mlua::Error>(_m)
        }?
    };
    {@-- ($lua: ident, $m: ident), $name: literal: $value: expr} => {
        $m.set($name, ($value).into_lua_entry(&$lua)?)?
    };
    {@-- ($lua: ident, $m: ident), $name: ident: $value: expr} => {
        $m.set(stringify!($name), ($value).into_lua_entry(&$lua)?)?
    };
}

pub trait LuaPrint<'a> {
    fn printable_value(&self) -> Result<String, LuaError>;
}

impl<'a> LuaPrint<'a> for mlua::Value<'a> {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(self, 0)
    }
}

impl<'a> LuaPrint<'a> for mlua::Table<'a> {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::Table(self.clone()), 0)
    }
}

impl<'a> LuaPrint<'a> for bool {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::Boolean(*self), 0)
    }
}
impl<'a> LuaPrint<'a> for mlua::Function<'a> {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::Function(self.clone()), 0)
    }
}
impl<'a> LuaPrint<'a> for mlua::Thread<'a> {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::Thread(self.clone()), 0)
    }
}

impl<'a> LuaPrint<'a> for mlua::String<'a> {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::String(self.clone()), 0)
    }
}

impl<'a> LuaPrint<'a> for mlua::Error {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::Error(self.clone()), 0)
    }
}

impl<'a> LuaPrint<'a> for mlua::Integer {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::Integer(*self), 0)
    }
}

impl<'a> LuaPrint<'a> for mlua::Number {
    fn printable_value(&self) -> Result<String, LuaError> {
        pformat(&mlua::Value::Number(*self), 0)
    }
}

impl<'a, T: ToString> LuaPrint<'a> for Option<T> {
    fn printable_value(&self) -> Result<String, LuaError> {
        match self {
            Some(v) => Ok(v.to_string()),
            None => pformat(&mlua::Value::Nil, 0),
        }
    }
}

#[macro_export]
macro_rules! lua_print {
    ($($arg: expr),*) => {
        {
            use $crate::lua::LuaPrint;
            println!("{}", vec![
                $($arg.printable_value().unwrap(),)*
            ].join(" "))
        }
    };
}
