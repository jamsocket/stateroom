mod client_socket_connection;
mod messages;
mod room_actor;
mod room_id;
mod server_state;
mod service_actor;
mod shutdown_policy;

pub use crate::room_id::{RoomIdGenerator, RoomIdStrategy, UuidRoomIdGenerator};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{self, get, post};
use actix_web::{web::Data, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
pub use client_socket_connection::ClientSocketConnection;
use jamsocket::JamsocketServiceBuilder;
pub use messages::{AssignUserId, MessageFromClient, MessageFromServer};
pub use room_actor::RoomActor;
use serde::{Deserialize, Serialize};
use server_state::ServerState;
use service_actor::ServiceActorContext;
pub use shutdown_policy::ServiceShutdownPolicy;
use std::time::{Duration, Instant};

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

    pub shutdown_policy: ServiceShutdownPolicy,
}

async fn new_room<T: JamsocketServiceBuilder<ServiceActorContext> + 'static + Clone>(
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let server_state: &Data<ServerState<T>> = req.app_data().unwrap();
    let room_id = server_state.new_room_generated().await?;

    Ok(HttpResponse::Ok().json(NewRoom { room_id }))
}

async fn new_room_explicit<T: JamsocketServiceBuilder<ServiceActorContext> + 'static + Clone>(
    req: HttpRequest,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let server_state: &Data<ServerState<T>> = req.app_data().unwrap();
    server_state.explicit_new_room(room_id.as_ref()).await?;

    Ok(HttpResponse::Ok().json(NewRoom {
        room_id: room_id.to_string(),
    }))
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

    let server_state: &Data<ServerState<T>> = req.app_data().unwrap();
    let room_addr = server_state.connect_room(room_id.as_ref()).await?;

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
            heartbeat_interval: server_state.settings.heartbeat_interval,
            heartbeat_timeout: server_state.settings.heartbeat_timeout,
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
    let host = format!("127.0.0.1:{}", server_settings.port);
    let room_mapper = Data::new(ServerState::new(host_factory, server_settings));

    actix_web::rt::System::new().block_on(async move {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(room_mapper.clone())
                .route("/status", get().to(status))
                .route("/new_room", post().to(new_room::<T>))
                .route("/ws/{room_id}", get().to(websocket::<T>))
                .route("/ws/{room_id}", post().to(new_room_explicit::<T>))
                // TODO: don't hard-code this
                .service(actix_files::Files::new("client/", "./static-client"))
                .service(actix_files::Files::new("/", "./static").index_file("index.html"))
        })
        .bind(&host)
        .unwrap();

        log::info!("Listening at {}", &host);
        server.run().await
    })
}
