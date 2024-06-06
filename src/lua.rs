use std::fmt::Display;
use std::path::PathBuf;

use mlua::FromLuaMulti;
use mlua::Function;
use mlua::IntoLua;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::Table;
use mlua::Error as LuaError;

use crate::modules::pformat;
use crate::modules::Import;

pub use crate::lua_print as print;
pub use crate::lua_table as module;
pub use crate::lua_table as table;
pub use crate::lua_array as array;
pub use crate::lua_multi as multi;
pub use crate::lua_pairs as __pairs;

pub const NIL: mlua::Value = mlua::Value::Nil;

pub trait IntoLuaEntry<'lua, R, L = ()> {
    fn into_lua_entry(self, lua: &'lua Lua) -> Result<mlua::Value<'lua>, mlua::Error>;
}

impl<'lua, A, R, F> IntoLuaEntry<'lua, Function<'lua>, (A, R)> for F
where
    A: FromLuaMulti<'lua>,
    R: IntoLuaMulti<'lua>,
    F: Fn(&'lua Lua, A) -> Result<R, mlua::Error> + Send + 'static,
{
    fn into_lua_entry(self, lua: &'lua Lua) -> Result<mlua::Value<'lua>, mlua::Error> {
       Ok(mlua::Value::Function(lua.create_function(self)?))
    }
}

impl<'lua, I: Import> IntoLuaEntry<'lua, Table<'lua>, ()> for I {
    fn into_lua_entry(self, _lua: &'lua Lua) -> Result<mlua::Value<'lua>, mlua::Error> {
        Ok(mlua::Value::Table(I::import(_lua)?))
    }
}

impl<'lua, I: IntoLua<'lua>> IntoLuaEntry<'lua, I, ()> for I {
    fn into_lua_entry(self, lua: &'lua Lua) -> Result<mlua::Value<'lua>, mlua::Error> {
        self.into_lua(lua)
    }
}

#[macro_export]
macro_rules! lua_multi {
    {
        [$lua: ident] $($value: expr),* $(,)?
    } => {
        mlua::MultiValue::from_vec(vec![
            $($value.into_lua($lua)?,)*    
        ])
    };
}

#[macro_export]
macro_rules! lua_table {
    {
        [$lua: ident] $($name: expr => $value: expr),* $(,)?
    } => {
        {
            use $crate::lua::IntoLuaEntry;
            $lua.create_table_from([$(
                ($name.to_string(), ($value).into_lua_entry(&$lua)?),
            )*])
        }
    };
}

#[macro_export]
macro_rules! lua_array {
    {
        [$lua: ident] $($value: expr),* $(,)?
    } => {
        {
            use $crate::lua::IntoLuaEntry;
            $lua.create_table_from([$(
                ($value).into_lua_entry(&$lua)?
            )*].iter().enumerate().map(|(i, v)| (i + 1, v)))
        }?
    };
}

pub trait LuaPrint<'a> {
    fn printable_value(&self) -> Result<String, LuaError>;
}

pub trait LuaFmt<'a> {
    fn lua_fmt(&self, pretty: bool, indent: usize) -> String;
}

impl<'a, F: LuaFmt<'a>> LuaFmt<'a> for &F {
    fn lua_fmt(&self, pretty: bool, indent: usize) -> String {
        (*self).lua_fmt(pretty, indent)
    }
}

impl<'a> LuaFmt<'a> for String {
    fn lua_fmt(&self, _: bool, _: usize) -> String {
        format!("\"{}\"", self)
    }
}

impl<'a> LuaFmt<'a> for &'a str {
    fn lua_fmt(&self, _: bool, _: usize) -> String {
        format!("\"{}\"", self)
    }
}

impl<'a> LuaFmt<'a> for bool {
    fn lua_fmt(&self, _: bool, _: usize) -> String {
        self.to_string()
    }
}

impl<'a> LuaFmt<'a> for PathBuf {
    fn lua_fmt(&self, _: bool, _: usize) -> String {
        format!("\"{}\"", self.display())
    }
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

impl<'a, T: Display> LuaPrint<'a> for Option<T> {
    fn printable_value(&self) -> Result<String, LuaError> {
        match self {
            Some(v) => Ok(format!("{:#}", v)),
            None => pformat(&mlua::Value::Nil, 0),
        }
    }
}

impl <'a, T: LuaFmt<'a>> LuaPrint<'a> for T {
    fn printable_value(&self) -> Result<String, LuaError> {
        Ok(self.lua_fmt(true, 0))
    }
}

#[macro_export]
macro_rules! lua_print {
    ($($arg: expr),* $(,)?) => {
        {
            use $crate::lua::LuaPrint;
            println!("{}", vec![
                $($arg.printable_value().unwrap(),)*
            ].join(" "))
        }
    };
}

pub struct LuaStructFormat {
    _s: Vec<(String, String)>,
    pretty: bool,
    indent: usize,
}

impl LuaStructFormat {
    pub fn new(pretty: bool, indent: usize) -> Self {
        Self { _s: Vec::new(), pretty, indent }
    }

    pub fn field<'lua, T: LuaFmt<'lua>, S: AsRef<str>>(mut self, name: S, format: T) -> Self {
        self._s.push((name.as_ref().to_string(), format.lua_fmt(self.pretty, self.indent+1)));
        self
    }
}

impl Display for LuaStructFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset = (0..self.indent).map(|_| "  ").collect::<String>();
        let nl = if self.pretty { format!("\n{offset}") } else { String::from(" ") };
        let spacing = if self.pretty { "  " } else { "" };
        write!(f, 
            "{{{nl}{spacing}{}{nl}}}",
            self._s.iter().map(|(k, v)| format!("{k} = {v}")).collect::<Vec<String>>().join(format!(",{nl}{spacing}").as_str()),
        )
    }
}

#[macro_export]
macro_rules! lua_pairs {
    {
        pub fn $methods: ident ::__pairs ($this: ident, key$(,$($arg: ident: $arg_type: ty),* $(,)?)?) {
            match key {
                None => ($next_key: expr, $next_value: expr),
                $($k: literal => ($nk: expr, $nv: expr)),*
                $(,)?
            }
        }
    } => {
        $methods.add_meta_method(mlua::MetaMethod::Pairs, |lua, this, ($($($arg,)*)?): ($($($arg_type)*)?)| {
            Ok($crate::lua::multi! { [lua]
                lua.create_function(|lua, ($this, key): (Self, Option<String>)| {
                    match key.as_deref() {
                        None => Ok($crate::lua::multi! { [lua] $next_key, $next_value }),
                        $(Some($k) => Ok($crate::lua::multi! { [lua] $nk, $nv }),)*
                        _ => Ok(_lua::multi! { [lua] mlua::Value::Nil, mlua::Value::Nil }),
                    }
                })?,
                this.clone(),
                mlua::Value::Nil,
            })
        });
    };
}
