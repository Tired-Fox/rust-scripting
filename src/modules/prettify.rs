use mlua::{Error as LuaError, Integer, Lua, Value, Variadic};

use super::Import;

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
                items.push(format!(
                    "{spacing}{} = {}",
                    pformat(&k, indent + 2)?,
                    pformat(&v, indent + 2)?
                ));
            }

            format!("{{\n  {}\n{spacing}}}", items.join(",\n  "))
        }
        Value::Function(f) => {
            let info = f.info();
            format!(
                "Function({}line: {})",
                info.name
                    .as_ref()
                    .map(|v| format!("{}, ", v))
                    .unwrap_or(String::new()),
                info.line_defined
                    .map(|v| v.to_string())
                    .unwrap_or("???".into())
            )
        }
        Value::Thread(_) => "Thread".into(),
        Value::UserData(_) => "Any".into(),
        _ => format!("{:?}", arg),
    })
}

pub struct Prettify;
impl Prettify {
    fn pprint(lua: &Lua, args: Variadic<Value>) -> mlua::Result<()> {
        for arg in args {
            println!("{}", Prettify::pstring(lua, (arg, None))?);
        }
        Ok(())
    }

    fn pstring(_: &Lua, (arg, indent): (Value, Option<Integer>)) -> Result<String, LuaError> {
        let indent = indent.unwrap_or(0) as usize;
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
                    items.push(format!(
                        "{spacing}{} = {}",
                        pformat(&k, indent + 2)?,
                        pformat(&v, indent + 2)?
                    ));
                }

                format!("{{\n  {}\n{spacing}}}", items.join(",\n  "))
            }
            Value::Function(f) => {
                let info = f.info();
                format!(
                    "Function({}line: {})",
                    info.name
                        .as_ref()
                        .map(|v| format!("{}, ", v))
                        .unwrap_or(String::new()),
                    info.line_defined
                        .map(|v| v.to_string())
                        .unwrap_or("???".into())
                )
            }
            Value::Thread(_) => "Thread".into(),
            Value::UserData(_) => "Any".into(),
            _ => format!("{:?}", arg),
        })
    }
}

impl Import for Prettify {
    fn module_name() -> &'static str {
        "pretty"
    }

    fn import(lua: &Lua) -> Result<mlua::prelude::LuaTable, LuaError> {
        let module = lua.create_table()?;
        module.set("print", lua.create_function(Prettify::pprint)?)?;
        module.set("stringify", lua.create_function(Prettify::pstring)?)?;
        Ok(module)
    }
}
