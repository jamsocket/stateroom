use crate::cli_opts::ServeCommand;
use crate::room_id::{RoomIdGenerator, UuidRoomIdGenerator};
use actix::{Actor, Addr};
use actix_web::{post, web::Data, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use async_std::sync::RwLock;
use jamsocket::JamsocketContext;
use jamsocket_server::{GetRoomAddr, RoomActor, ServiceActor};
use jamsocket_wasm::WasmHost;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use wasmtime::{Engine, Module};

type RoomMapper = RwLock<HashMap<String, Addr<RoomActor>>>;

#[derive(Serialize, Deserialize)]
struct NewRoom {
    room_id: String,
}

#[derive(Clone)]
pub struct Instantiator {
    engine: Arc<Engine>,
    module: Arc<Module>,
}

impl Instantiator {
    fn create_room(&self, wctx: impl JamsocketContext + Send + Sync + 'static) -> WasmHost {
        let engine = self.engine.clone();
        let module = self.module.clone();

        WasmHost::new(module.as_ref(), engine.as_ref(), Arc::new(wctx)).unwrap()
    }

    fn new(wasm_file: &str) -> Self {
        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_file).unwrap();

        Instantiator {
            engine: Arc::new(engine),
            module: Arc::new(module),
        }
    }
}

#[post("/new_room")]
async fn new_room(req: HttpRequest) -> Result<HttpResponse, Error> {
    let instantiator: &Data<Instantiator> = req.app_data().unwrap();
    let it2 = instantiator.clone();

    let service_constructor = Box::new(move |wctx| it2.create_room(wctx));

    let room_id = UuidRoomIdGenerator.generate();
    let room_addr =
        ServiceActor::create(|ctx| ServiceActor::new(ctx, service_constructor).unwrap())
            .send(GetRoomAddr)
            .await
            .unwrap();

    let room_mapper: &Data<RoomMapper> = req.app_data().unwrap();
    room_mapper.write().await.insert(room_id.clone(), room_addr);

    Ok(HttpResponse::Ok().json(NewRoom { room_id }))
}

pub fn serve(serve_opts: ServeCommand) -> std::io::Result<()> {
    let ServeCommand {
        module,
        port,
        rooms,
    } = serve_opts;

    actix_web::rt::System::new().block_on(async move {
        let server =
            HttpServer::new(move || App::new().app_data(RoomMapper::default()).service(new_room))
                .bind(&format!("127.0.0.1:{}", port))
                .unwrap();

        server.run().await
    })
}
