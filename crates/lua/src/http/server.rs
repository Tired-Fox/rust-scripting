use hyper::body::Incoming;
use hyper::Request;
use mlua::{AnyUserData, Function, Lua, Table, UserData, UserDataMethods};
use mlua::ffi::lua_WarnFunction;

struct LuaRequest(Request<Incoming>);
impl UserData for LuaRequest {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("method", |_lua, req, ()| Ok((req.0).method().to_string()));
    }
}

#[derive(Debug, Clone)]
pub struct Server(pub &'static Lua);

impl UserData for Server {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Constructor
        // methods.add_function("new", move |lua, ()| {
        //     Ok(Server(lua))
        // })
        methods.add_function("get", |lua, (this, url, handler): (AnyUserData, String, Function)| {
            println!("GET {}", url);
            let table = match this.user_value::<Option<Table>>()? {
                Some(data) => data,
                None => {
                    let table = lua.create_table()?;
                    this.set_user_value(table.clone())?;
                    table
                }
            };

            let get = match table.get::<_, Option<Table>>("get")? {
                Some(get) => get,
                None => {
                    let get = lua.create_table()?;
                    table.set("get", get.clone())?;
                    get
                }
            };

            let ilen = get.raw_len();
            let table = lua.create_table()?;
            table.set("path", url)?;
            table.set("handler", handler)?;
            get.set(ilen + 1, table)
        })
    }
}