use mlua::{Error as LuaError, Integer, Lua, Table, Value, Variadic};

use super::Import;

pub fn pformat(arg: &Value, indent: usize) -> Result<String, LuaError> {
    let spacing = " ".repeat(indent);
    Ok(match arg {
        Value::Nil => "nil".to_string(),
        Value::Boolean(bool) => format!("{}", bool),
        Value::LightUserData(_) => "Pointer".into(),
        Value::Integer(i) => format!("{:?}", i),
        Value::Number(n) => format!("{:?}", n),
        Value::String(s) => s.to_str().unwrap_or("").into(),
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
        Value::Error(e) => format!("{}", e),
    })
}

pub struct Prettify;
impl Prettify {
    pub fn pprint(_: &Lua, args: Variadic<Value>) -> mlua::Result<()> {
        let args = args
            .iter()
            .map(|v| pformat(v, 0))
            .collect::<Result<Vec<String>, LuaError>>()?.join(" ");
        println!("{}", args);
        Ok(())
    }

    fn pstring(_: &Lua, (arg, indent): (Value, Option<Integer>)) -> Result<String, LuaError> {
        pformat(&arg, indent.unwrap_or(0) as usize)
    }
}

impl Import for Prettify {
    fn module_name() -> &'static str {
        "pretty"
    }

    fn extend(table: &Table<'_>, lua: &Lua) -> Result<(), LuaError> {
        table.set("print", lua.create_function(Prettify::pprint)?)?;
        table.set("stringify", lua.create_function(Prettify::pstring)?)?;
        Ok(())
    }
}
