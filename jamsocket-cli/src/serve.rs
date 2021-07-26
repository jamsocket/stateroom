use crate::cli_opts::ServeCommand;
use crate::room_id::{RoomIdGenerator, UuidRoomIdGenerator};
use actix::{Actor, Addr};
use actix_web::{post, web::Data, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use async_std::sync::RwLock;
use jamsocket_server::{GetRoomAddr, RoomActor, ServiceActor};
use jamsocket_wasm_host::WasmHostFactory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type RoomMapper = RwLock<HashMap<String, Addr<RoomActor>>>;

#[derive(Serialize, Deserialize)]
struct NewRoom {
    room_id: String,
}

#[post("/new_room")]
async fn new_room(req: HttpRequest) -> Result<HttpResponse, Error> {
    let instantiator: &Data<WasmHostFactory> = req.app_data().unwrap();
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
