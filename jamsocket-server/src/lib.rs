mod client_socket_connection;
mod messages;
mod room_actor;
mod room_id;
mod service_actor;

pub use crate::room_id::{RoomIdGenerator, RoomIdStrategy, UuidRoomIdGenerator};
use actix::{Actor, Addr, AsyncContext};
use actix_web::error::{ErrorBadRequest, ErrorConflict, ErrorInternalServerError};
use actix_web::web::{self, get, post};
use actix_web::{web::Data, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use async_std::sync::RwLock;
pub use client_socket_connection::ClientSocketConnection;
use jamsocket::JamsocketServiceBuilder;
pub use messages::{AssignUserId, MessageFromClient, MessageFromServer};
pub use room_actor::RoomActor;
use serde::{Deserialize, Serialize};
use service_actor::{ServiceActor, ServiceActorContext};
use std::collections::HashMap;
use std::time::{Duration, Instant};

type RoomMapper = RwLock<HashMap<String, Addr<RoomActor>>>;

#[derive(Serialize, Deserialize)]
struct NewRoom {
    room_id: String,
}

async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("ok"))
}

/// Settings used by the server.
pub struct ServerSettings {
    /// The duration of time between server-initiated WebSocket heartbeats.
    pub heartbeat_interval: Duration,

    /// The minimum amount of time between client heartbeats before a connection is dropped.
    pub heartbeat_timeout: Duration,

    /// The method by which new rooms are created and assigned names.
    pub room_id_strategy: RoomIdStrategy,

    /// The port to run the server on.
    pub port: u32,
}

async fn create_room<T: JamsocketServiceBuilder<ServiceActorContext> + Clone>(
    room_id: String,
    room_mapper: &RoomMapper,
    host_factory: &Data<T>,
) -> (bool, Addr<RoomActor>) {
    match room_mapper.write().await.entry(room_id.clone()) {
        std::collections::hash_map::Entry::Occupied(entry) => (true, entry.get().clone()),
        std::collections::hash_map::Entry::Vacant(entry) => {
            let host_factory: T = host_factory.get_ref().clone();

            let room_actor = {
                let room_id = room_id.clone();

                RoomActor::create(|room_actor_context| {
                    let service_actor = ServiceActor::create(|service_actor_context| {
                        ServiceActor::new(
                            service_actor_context,
                            room_id.clone(),
                            host_factory,
                            room_actor_context.address().recipient(),
                        )
                        .unwrap()
                    });

                    RoomActor::new(room_id, service_actor.recipient())
                })
            };

            entry.insert(room_actor.clone());

            log::info!("Created room: {}", &room_id);
            (false, room_actor)
        }
    }
}

async fn new_room<T: JamsocketServiceBuilder<ServiceActorContext> + 'static + Clone>(
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let wasm_host_factory: &Data<T> = req.app_data().unwrap();
    let server_settings: &Data<ServerSettings> = req.app_data().unwrap();
    let room_mapper: &Data<RoomMapper> = req.app_data().unwrap();

    for _ in 0..100 {
        let room_id = {
            match &server_settings.room_id_strategy {
                RoomIdStrategy::Generator(g) => g.generate(),
                RoomIdStrategy::Implicit => UuidRoomIdGenerator.generate(),
                _ => {
                    return Err(ErrorBadRequest(
                        "Room ID strategy does not support room ID generation.",
                    ))
                }
            }
        };
    
        let (already_existed, _) = create_room(room_id.clone(), room_mapper, wasm_host_factory).await;

        if !already_existed {
            return Ok(HttpResponse::Ok().json(NewRoom {room_id}));
        }
    }

    Err(ErrorInternalServerError("Could not assign a unique room ID."))
}

async fn new_room_explicit<T: JamsocketServiceBuilder<ServiceActorContext> + 'static + Clone>(
    req: HttpRequest,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let wasm_host_factory: &Data<T> = req.app_data().unwrap();
    let server_settings: &Data<ServerSettings> = req.app_data().unwrap();
    let room_mapper: &Data<RoomMapper> = req.app_data().unwrap();

    let room_creation_allowed = match &server_settings.room_id_strategy {
        RoomIdStrategy::Explicit => true,
        RoomIdStrategy::Implicit => true,
        _ => false,
    };

    if room_creation_allowed {
        let (already_existed, _) = create_room(room_id.clone(), room_mapper, wasm_host_factory).await;
        if !already_existed {
            Ok(HttpResponse::Ok().json(NewRoom {room_id: room_id.to_owned()}))
        } else {
            Err(ErrorConflict("Attempted to create a room that already exists."))
        }
    } else {
        Err(ErrorInternalServerError("Explicit room creation is not enabled."))
    }
}

async fn websocket<T: JamsocketServiceBuilder<ServiceActorContext> + 'static + Clone>(
    req: HttpRequest,
    stream: web::Payload,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let ip = if let Some(peer_addr) = req.peer_addr() {
        peer_addr.ip().to_string()
    } else {
        "<unknown>".to_string()
    };

    let room_mapper: &Data<RoomMapper> = req.app_data().unwrap();
    let server_settings: &Data<ServerSettings> = req.app_data().unwrap();

    let maybe_room_addr = { room_mapper.read().await.get(room_id.as_ref()).cloned() };

    let room_addr = if let Some(room_addr) = maybe_room_addr {
        room_addr.clone()
    } else {
        let server_settings: &Data<ServerSettings> = req.app_data().unwrap();
        if let RoomIdStrategy::Implicit = server_settings.room_id_strategy {
            let wasm_host_factory: &Data<T> = req.app_data().unwrap();
            let (_, room_addr) = create_room(room_id.to_string(), room_mapper, wasm_host_factory).await;
            room_addr
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
            ip: ip.clone(),
            room_id: room_id.clone(),
            last_seen: Instant::now(),
            heartbeat_interval: server_settings.heartbeat_interval,
            heartbeat_timeout: server_settings.heartbeat_timeout,
            interval_handle: None,
        },
        &req,
        stream,
    ) {
        Ok((addr, resp)) => {
            log::info!(
                "New connection from IP {} to room {} (user {})",
                &ip,
                &room_id,
                user
            );
            room_addr.do_send(MessageFromClient::Connect(user, addr.recipient()));

            Ok(resp)
        }
        Err(e) => Err(e),
    }
}

/// Start a server given a cloneable [JamsocketServiceBuilder] and a [ServerSettings] object.
///
/// This function blocks until the server is terminated. While it is running, the following
/// endpoints are available:
/// - `/` (GET): return HTTP 200 if the server is running (useful as a baseline status check)
/// - `/new_room` (POST): create a new room, if not in `explicit` room creation mode.
/// - `/ws/{room_id}` (GET): initiate a WebSocket connection to the given room. If the room
///     does not exist and the server is in `implicit` room creation mode, it will be created.
pub fn do_serve<T: JamsocketServiceBuilder<ServiceActorContext> + Send + Sync + 'static + Clone>(
    host_factory: T,
    server_settings: ServerSettings,
) -> std::io::Result<()> {
    let room_mapper = Data::new(RoomMapper::default());
    let host_factory = Data::new(host_factory);
    let host = format!("127.0.0.1:{}", server_settings.port);
    let server_settings = Data::new(server_settings);

    actix_web::rt::System::new().block_on(async move {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(room_mapper.clone())
                .app_data(host_factory.clone())
                .app_data(server_settings.clone())
                // TODO: don't hard-code this
                .service(actix_files::Files::new("client/", "./static-client"))
                .service(actix_files::Files::new("/", "./static").index_file("index.html"))
                .route("/status", get().to(status))
                .route("/new_room", post().to(new_room::<T>))
                .route("/ws/{room_id}", get().to(websocket::<T>))
                .route("/ws/{room_id}", post().to(new_room_explicit::<T>))
        })
        .bind(&host)
        .unwrap();

        log::info!("Listening at {}", &host);
        server.run().await
    })
}
