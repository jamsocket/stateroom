use crate::cli_opts::ServeCommand;
use crate::room_id::{RoomIdGenerator, RoomIdStrategy, UuidRoomIdGenerator};
use actix::{Actor, Addr};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::web;
use actix_web::{get, post, web::Data, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use async_std::sync::RwLock;
use jamsocket_server::{
    AssignUserId, ClientSocketConnection, GetRoomAddr, MessageFromClient, RoomActor, ServiceActor,
    ServiceActorContext,
};
use jamsocket_wasm_host::{WasmHost, WasmHostFactory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type RoomMapper = RwLock<HashMap<String, Addr<RoomActor>>>;

#[derive(Serialize, Deserialize)]
struct NewRoom {
    room_id: String,
}

#[get("/")]
async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("ok"))
}

async fn try_create_room(
    room_id: String,
    room_mapper: &RoomMapper,
    host_factory: &Data<WasmHostFactory>,
) -> Option<Addr<RoomActor>> {
    match room_mapper.write().await.entry(room_id) {
        std::collections::hash_map::Entry::Occupied(_) => None,
        std::collections::hash_map::Entry::Vacant(entry) => {
            let service_constructor: Box<dyn FnOnce(ServiceActorContext) -> WasmHost> = {
                let wasm_host_factory = host_factory.clone();
                Box::new(move |wctx| wasm_host_factory.create_room(wctx))
            };

            let room_addr =
                ServiceActor::create(|ctx| ServiceActor::new(ctx, service_constructor).unwrap())
                    .send(GetRoomAddr)
                    .await
                    .unwrap();

            entry.insert(room_addr.clone());

            Some(room_addr)
        }
    }
}

#[post("/new_room")]
async fn new_room(req: HttpRequest) -> Result<HttpResponse, Error> {
    let wasm_host_factory: &Data<WasmHostFactory> = req.app_data().unwrap();

    let room_id = {
        let room_id_strategy: &Data<RoomIdStrategy> = req.app_data().unwrap();

        match &room_id_strategy.get_ref() {
            &RoomIdStrategy::Generator(g) => g.generate(),
            &RoomIdStrategy::Implicit => UuidRoomIdGenerator.generate(),
            _ => {
                return Err(ErrorBadRequest(
                    "Room ID strategy does not support room ID generation.",
                ))
            }
        }
    };

    let room_mapper: &Data<RoomMapper> = req.app_data().unwrap();
    // TODO: this could fail, we really need to wrap it in a `try_generate_room`.
    try_create_room(room_id.clone(), room_mapper, wasm_host_factory)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(NewRoom { room_id }))
}

#[get("/ws/{room_id}")]
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let room_mapper: &Data<RoomMapper> = req.app_data().unwrap();

    let room_addr = if let Some(room_addr) = room_mapper.read().await.get(room_id.as_ref()) {
        room_addr.clone()
    } else {
        let room_id_strategy: &Data<RoomIdStrategy> = req.app_data().unwrap();

        if let &RoomIdStrategy::Implicit = room_id_strategy.get_ref() {
            // TODO: there is technically a race condition where if a room does not exist when
            // we first try to read it, but it is created before we get the write lock, we will
            // fail to connect to the room when the correct behavior is to connect to the existing
            // room.
            let wasm_host_factory: &Data<WasmHostFactory> = req.app_data().unwrap();
            let room_addr = try_create_room(room_id.to_string(), room_mapper, wasm_host_factory);

            room_addr.await.unwrap()
        } else {
            return Err(ErrorBadRequest("The requested room was not found."));
        }
    };

    let user = room_addr
        .send(AssignUserId)
        .await
        .map_err(|_| ErrorInternalServerError("Error getting room."))?;

    match ws::start_with_addr(
        ClientSocketConnection {
            room: room_addr.clone().recipient(),
            user,
        },
        &req,
        stream,
    ) {
        Ok((addr, resp)) => {
            room_addr.do_send(MessageFromClient::Connect(user, addr.recipient()));

            Ok(resp)
        }
        Err(e) => Err(e),
    }
}

pub fn serve(serve_opts: ServeCommand) -> std::io::Result<()> {
    let ServeCommand {
        module,
        port,
        rooms,
    } = serve_opts;

    let host_factory = Data::new(WasmHostFactory::new(&module));
    let room_mapper = Data::new(RoomMapper::default());
    let room_id_strategy = Data::new(rooms);

    actix_web::rt::System::new().block_on(async move {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(room_mapper.clone())
                .app_data(host_factory.clone())
                .app_data(room_id_strategy.clone())
                .service(status)
                .service(new_room)
                .service(websocket)
        })
        .bind(&format!("127.0.0.1:{}", port))
        .unwrap();

        server.run().await
    })
}
