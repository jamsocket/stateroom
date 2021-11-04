mod client_socket_connection;
mod messages;
mod room_actor;
mod server_state;
mod service_actor;

use actix_web::error::ErrorInternalServerError;
use actix_web::web::{self, get};
use actix_web::{web::Data, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
pub use client_socket_connection::ClientSocketConnection;
use jamsocket::JamsocketServiceFactory;
pub use messages::{AssignClientId, MessageFromClient, MessageFromServer};
pub use room_actor::RoomActor;
use server_state::ServerState;
pub use service_actor::{ServiceActor, ServiceActorContext};
use std::time::{Duration, Instant};
use tracing_actix_web::TracingLogger;
use serde_json::{Value, json};

const DEFAULT_IP: &str = "0.0.0.0";

#[allow(clippy::unused_async)]
async fn status() -> Result<web::Json<Value>, Error> {
    Ok(web::Json(json!({
        "status": "ok"
    })))
}

/// Settings used by the server.
pub struct Server {
    /// The duration of time between server-initiated WebSocket heartbeats.
    ///
    /// Defaults to 30 seconds.
    pub heartbeat_interval: Duration,

    /// The minimum amount of time between client heartbeats before a connection is dropped.
    ///
    /// Defaults to 5 minutes.
    pub heartbeat_timeout: Duration,

    /// The port to run the server on. Defaults to 8080.
    pub port: u32,

    /// The IP to listen on. Defaults to 0.0.0.0.
    pub ip: String,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            heartbeat_interval: Duration::from_secs(30),
            heartbeat_timeout: Duration::from_secs(300),
            port: 8080,
            ip: DEFAULT_IP.to_string(),
        }
    }
}

impl Server {
    #[must_use]
    pub fn new() -> Self {
        Server::default()
    }

    #[must_use]
    pub fn with_heartbeat_interval(mut self, duration_seconds: u64) -> Self {
        self.heartbeat_interval = Duration::from_secs(duration_seconds);
        self
    }

    #[must_use]
    pub fn with_heartbeat_timeout(mut self, duration_seconds: u64) -> Self {
        self.heartbeat_timeout = Duration::from_secs(duration_seconds);
        self
    }

    #[must_use]
    pub fn with_port(mut self, port: u32) -> Self {
        self.port = port;
        self
    }

    #[must_use]
    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip = ip;
        self
    }

    /// Start a server given a [JamsocketService].
    ///
    /// This function blocks until the server is terminated. While it is running, the following
    /// endpoints are available:
    /// - `/` (GET): return HTTP 200 if the server is running (useful as a baseline status check)
    /// - `/new_room` (POST): create a new room, if not in `explicit` room creation mode.
    /// - `/ws/{room_id}` (GET): initiate a WebSocket connection to the given room. If the room
    ///     does not exist and the server is in `implicit` room creation mode, it will be created.
    pub fn serve(
        self,
        service_factory: impl JamsocketServiceFactory<ServiceActorContext>,
    ) -> std::io::Result<()> {
        let host = format!("{}:{}", self.ip, self.port);

        actix_web::rt::System::new().block_on(async move {
            let server_state = Data::new(ServerState::new(service_factory, self).unwrap());

            let server = HttpServer::new(move || {
                #[allow(unused_mut)] // mut only needed with crate feature `serve-static`.
                let mut app = App::new()
                    .app_data(server_state.clone())
                    .route("/status", get().to(status))
                    .route("/ws", get().to(websocket))
                    .wrap(TracingLogger::default());

                app
            })
            .bind(&host)?;

            tracing::info!(%host, "Server is listening");
            server.run().await
        })
    }
}

async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
) -> actix_web::Result<HttpResponse> {
    let ip = req
        .peer_addr()
        .map_or_else(|| "<unknown>".to_string(), |a| a.ip().to_string());

    let server_state: &Data<ServerState> = req.app_data().expect("Could not load ServerState.");

    let room_addr = server_state.room_addr.clone();
    let client_id = room_addr
        .send(AssignClientId)
        .await
        .map_err(|_| ErrorInternalServerError("Error getting room."))?;

    match ws::start_with_addr(
        ClientSocketConnection {
            room: room_addr.clone().recipient(),
            client_id,
            ip: ip.clone(),
            last_seen: Instant::now(),
            heartbeat_interval: server_state.settings.heartbeat_interval,
            heartbeat_timeout: server_state.settings.heartbeat_timeout,
            interval_handle: None,
        },
        &req,
        stream,
    ) {
        Ok((addr, resp)) => {
            tracing::info!(
                %ip,
                ?client_id,
                "New connection",
            );
            room_addr.do_send(MessageFromClient::Connect(client_id, addr.recipient()));

            Ok(resp)
        }
        Err(e) => Err(e),
    }
}
