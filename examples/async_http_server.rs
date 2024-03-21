use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response};

use hyper_util::rt::TokioIo;
use mlua::{
    chunk, Error as LuaError, Function, Lua, String as LuaString, Table, UserData, UserDataMethods,
};
use tokio::net::TcpListener;

struct LuaRequest(Request<Incoming>);

impl UserData for LuaRequest {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("method", |_lua, req, ()| Ok((req.0).method().to_string()));
    }
}

#[derive(Clone)]
pub struct Svc(&'static Lua);

impl Service<Request<Incoming>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = LuaError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        // If handler returns an error then generate 5xx response
        let lua = self.0;
        let lua_req = LuaRequest(req);
        Box::pin(async move {
            let handler: Function = lua.named_registry_value("http_handler")?;
            match handler.call_async::<_, Table>(lua_req).await {
                Ok(lua_resp) => {
                    let status = lua_resp.get::<_, Option<u16>>("status")?.unwrap_or(200);
                    let mut resp = Response::builder().status(status);

                    // Set headers
                    if let Some(headers) = lua_resp.get::<_, Option<Table>>("headers")? {
                        for pair in headers.pairs::<String, LuaString>() {
                            let (h, v) = pair?;
                            resp = resp.header(&h, v.as_bytes());
                        }
                    }

                    let body = lua_resp
                        .get::<_, Option<LuaString>>("body")?
                        .map(|b| Full::new(Bytes::from(b.as_bytes().to_vec())))
                        .unwrap_or_else(Full::default);

                    Ok(resp.body(body).unwrap())
                }
                Err(err) => {
                    eprintln!("{}", err);
                    Ok(Response::builder()
                        .status(500)
                        .body(Full::new(Bytes::from("Internal Server Error")))
                        .unwrap())
                }
            }
        })
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let lua = Lua::new();

    // Create Lua handler function
    let handler: Function = lua
        .load(chunk! {
            function(req)
                return {
                    status = 200,
                    headers = {
                        ["X-Req-Method"] = req:method(),
                    },
                    body = "Hello from Lua!\n"
                }
            end
        })
        .eval()
        .expect("cannot create Lua handler");

    // Store it in the Registry
    lua.set_named_registry_value("http_handler", handler)
        .expect("cannot store Lua handler");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let local = tokio::task::LocalSet::new();
    local
        .run_until(async move {
            let svc = Svc(Box::leak(Box::new(lua)));

            loop {
                let (stream, _) = listener.accept().await?;
                let io = TokioIo::new(stream);

                let service = svc.clone();
                tokio::task::spawn_local(async move {
                    if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
        })
        .await
}
