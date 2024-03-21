// TODO: Re-write for latest hyper lib
use std::collections::HashMap;

use http_body_util::{BodyExt, Empty};
use hyper::{
    body::{Bytes, Incoming},
    header::HOST,
    Request,
};

use hyper_util::rt::TokioIo;
use mlua::{chunk, ExternalResult, Lua, Result, UserData, UserDataMethods};
use tokio::net::TcpStream;

struct BodyReader(Incoming);

impl UserData for BodyReader {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("read", |lua, reader, ()| async move {
            let mut buffer: Vec<u8> = Vec::new();
            while let Some(next) = reader.0.frame().await {
                let frame = next.into_lua_err()?;
                if let Some(chunk) = frame.data_ref() {
                    buffer.extend(chunk.to_vec().iter())
                }
            }

            return Some(lua.create_string(buffer)).transpose();
        });
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let lua = Lua::new();

    let fetch_url = lua.create_async_function(|lua, uri: String| async move {
        let url = uri.parse::<hyper::Uri>().into_lua_err()?;

        let host = url.host().expect("uri has no host");
        let port = url.port_u16().unwrap_or(80);

        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(address).await.into_lua_err()?;

        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io)
            .await
            .into_lua_err()?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Connection failed: {:?}", err);
            }
        });

        let authority = url.authority().unwrap().clone();
        let req = Request::builder()
            .uri(uri)
            .header(HOST, authority.as_str())
            .body(Empty::<Bytes>::default())
            .into_lua_err()?;

        let resp = sender.send_request(req).await.into_lua_err()?;

        let lua_resp = lua.create_table()?;
        lua_resp.set("status", resp.status().as_u16())?;

        let mut headers = HashMap::new();
        for (key, value) in resp.headers() {
            headers
                .entry(key.as_str())
                .or_insert(Vec::new())
                .push(value.to_str().into_lua_err()?);
        }

        lua_resp.set("headers", headers)?;
        lua_resp.set("body", BodyReader(resp.into_body()))?;

        Ok(lua_resp)
    })?;

    let http = lua.create_table()?;
    http.set("get", fetch_url)?;

    let globals = lua.globals();
    globals.set("http", http)?;

    let f = lua
        .load(chunk! {
            local res = http.get("http://httpbin.org/ip")
            local res = http.post("http://httpbin.org/ip")

            print("status: "..res.status)
            for key, vals in pairs(res.headers) do
                for _, val in ipairs(vals) do
                    print(key..": "..val)
                end
            end
            local body = res.body:read()
            if body then
                print(body)
            end
        })
        .into_function()?;

    f.call_async(()).await
}
