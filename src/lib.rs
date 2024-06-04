pub mod modules;
pub mod prelude;
pub mod lua;

use mlua::{Error as LuaError, Lua, Table};

pub type Result<T> = std::result::Result<T, LuaError>;

#[macro_export]
macro_rules! splat {
    ($($arg: expr),*) => {
        mlua::Variadic::from_iter([$($arg,)*])
    };
}

pub trait LuaExt {
    fn path(&self) -> String;
    fn append_path(&mut self, path: &str);
    fn append_paths(&mut self, paths: &[&str]);
    fn set_path(&mut self, path: &str);
    fn set_paths(&mut self, paths: &[&str]);
}

impl LuaExt for Lua {
    /// Get the `package.path` value
    ///
    /// This is the value used by the lua engine to resolve `require` calls.
    /// see: 
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.path
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.searchpath
    fn path(&self) -> String {
        self.globals()
            .get::<_, Table>("package").unwrap()
            .get::<_, String>("path").unwrap()
    }

    /// Set the `package.path` value
    ///
    /// This is the value used by the lua engine to resolve `require` calls.
    /// see: 
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.path
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.searchpath
    fn set_path(&mut self, path: &str) {
        self.globals().get::<_, Table>("package").unwrap().set("path", path).unwrap();
    }

    /// Set the `package.path` values
    ///
    /// This is the value used by the lua engine to resolve `require` calls.
    /// see: 
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.path
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.searchpath
    fn set_paths(&mut self, paths: &[&str]) {
        self.globals().get::<_, Table>("package").unwrap().set("path", paths.join(";")).unwrap();
    }

    /// Append a path tothe `package.path` value
    ///
    /// This is the value used by the lua engine to resolve `require` calls.
    /// see: 
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.path
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.searchpath
    fn append_path(&mut self, path: &str) {
        let mut lua_path = self.path().split(';').map(ToString::to_string).collect::<Vec<_>>();
        lua_path.push(path.to_string());
        self.globals().get::<_, Table>("package").unwrap().set("path", lua_path.join(";")).unwrap();
    }

    /// Append paths to the `package.path` value
    ///
    /// This is the value used by the lua engine to resolve `require` calls.
    /// see: 
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.path
    ///   - https://www.lua.org/manual/5.4/manual.html#pdf-package.searchpath
    fn append_paths(&mut self, paths: &[&str]) {
        let mut lua_path = self.path().split(';').map(ToString::to_string).collect::<Vec<_>>();
        lua_path.extend(paths.iter().map(|s| s.to_string()));
        self.globals().get::<_, Table>("package").unwrap().set("path", lua_path.join(";")).unwrap();
    }
}
