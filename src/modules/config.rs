use std::{path::PathBuf, sync::{Arc, Mutex}};

use mlua::{FromLua, Lua, MetaMethod, UserData, UserDataFields};
use serde::ser::Error;

use crate::lua::{LuaFmt, LuaStructFormat};


#[derive(Debug, Clone, Default)]
pub struct Paths {
    pub projects: PathBuf,
    pub download: PathBuf,
    pub build: PathBuf,
}

impl<'lua> LuaFmt<'lua> for Paths {
    fn lua_fmt(&self, pretty: bool, indent: usize) -> String {
        LuaStructFormat::new(pretty, indent)
            .field("projects", &self.projects)
            .field("download", &self.download)
            .field("build", &self.build)
            .to_string()
    }
}

impl<'lua> FromLua<'lua> for Paths {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(table) => Ok(Paths {
                projects: table.get::<_, String>("projects").unwrap_or(String::new()).into(),
                download: table.get::<_, String>("download").unwrap_or(String::new()).into(),
                build: table.get::<_, String>("build").unwrap_or(String::new()).into(),
            }),
            mlua::Value::UserData(paths) => {
                let paths = paths.borrow::<Paths>()?;
                Ok(Paths {
                    projects: paths.projects.clone(),
                    download: paths.download.clone(),
                    build: paths.build.clone(),
                })
            },
            _ => Err(mlua::Error::custom(format!("Paths must be a table or userdata; was {:?}", value)))
        }
    }
}

impl UserData for Paths {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, pretty: Option<bool>| {
            Ok(this.lua_fmt(pretty.unwrap_or(false), 0))
        })
    }

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("projects", |_, this: &Self| Ok(this.projects.display().to_string()));
        fields.add_field_method_set("projects", |_, this: &mut Self, new: String| {
            this.projects = PathBuf::from(new);
            Ok(())
        });
        fields.add_field_method_get("download", |_, this: &Self| Ok(this.download.display().to_string()));
        fields.add_field_method_set("download", |_, this: &mut Self, new: String| {
            this.download = PathBuf::from(new);
            Ok(())
        });
        fields.add_field_method_get("build", |_, this: &Self| Ok(this.build.display().to_string()));
        fields.add_field_method_set("build", |_, this: &mut Self, new: String| {
            this.build = PathBuf::from(new);
            Ok(())
        });
    }
}

#[derive(Debug, Clone, Default)]
pub struct Features {
    pub show_docker_logs: bool,
}

impl<'lua> LuaFmt<'lua> for Features {
    fn lua_fmt(&self, pretty: bool, indent: usize) -> String {
        LuaStructFormat::new(pretty, indent)
            .field("show_docker_logs", self.show_docker_logs)
            .to_string()
    }
}

impl<'lua> FromLua<'lua> for Features {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(table) => Ok(Features {
                show_docker_logs: table.get::<_, bool>("show_docker_logs").unwrap_or(false),
            }),
            mlua::Value::UserData(features) => {
                let features = features.borrow::<Features>()?;
                Ok(Features {
                    show_docker_logs: features.show_docker_logs,
                })
            },
            _ => Err(mlua::Error::custom(format!("Features must be a table or userdata; was {:?}", value)))
        }
    }
}

impl UserData for Features {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, pretty: Option<bool>| {
            Ok(this.lua_fmt(pretty.unwrap_or(false), 0))
        })
    }

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("show_docker_logs", |_, this: &Self| Ok(this.show_docker_logs));
        fields.add_field_method_set("show_docker_logs", |_, this: &mut Self, new: bool| {
            this.show_docker_logs = new;
            Ok(())
        });
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub paths: Arc<Mutex<Paths>>,
    pub features: Arc<Mutex<Features>>,
}

impl<'lua> LuaFmt<'lua> for Config {
    fn lua_fmt(&self, pretty: bool, indent: usize) -> String {
        LuaStructFormat::new(pretty, indent)
            .field("paths", &*self.paths.lock().unwrap())
            .field("features", &*self.features.lock().unwrap())
            .to_string()
    }
}

impl<'lua> FromLua<'lua> for Config {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(table) => Ok(Config {
                paths: Arc::new(Mutex::new(table.get::<_, Paths>("paths").unwrap_or(Paths::default()))),
                features: Arc::new(Mutex::new(table.get::<_, Features>("features").unwrap_or(Features::default()))),
            }),
            mlua::Value::UserData(config) => {
                let config = config.borrow::<Config>()?;
                Ok(Config {
                    paths: config.paths.clone(),
                    features: config.features.clone(),
                })
            },
            _ => Err(mlua::Error::custom(format!("Config must be a table or userdata; was {:?}", value)))
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: Arc::new(Mutex::new(Paths::default())),
            features: Arc::new(Mutex::new(Features::default())),
        }
    }
}

impl UserData for Config {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, pretty: Option<bool>| {
            Ok(this.lua_fmt(pretty.unwrap_or(false), 0))
        })
    }

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("paths", |_, this: &Self| Ok(this.paths.clone()));
        fields.add_field_method_set("paths", |_, this: &mut Self, new: Paths| {
            let paths = &mut (*this.paths.lock().unwrap());
            *paths = new;
            Ok(())
        });

        fields.add_field_method_get("features", |_, this: &Self| Ok(this.features.clone()));
        fields.add_field_method_set("features", |_, this: &mut Self, new: Features| {
            let features = &mut (*this.features.lock().unwrap());
            *features = new;
            Ok(())
        });
    }
}
