mod client_socket_connection;
mod messages;
mod room_actor;
mod room_id;
mod server_state;
mod service_actor;
mod shutdown_policy;

pub use crate::room_id::{
    RoomIdGenerator, RoomIdStrategy, ShortRoomIdGenerator, ShortRoomIdGeneratorFactory,
    UuidRoomIdGenerator, UuidRoomIdGeneratorFactory,
};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{self, get, post};
use actix_web::{web::Data, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
pub use client_socket_connection::ClientSocketConnection;
use jamsocket::{JamsocketServiceFactory, SimpleJamsocketService, SimpleJamsocketServiceFactory};
pub use messages::{AssignClientId, MessageFromClient, MessageFromServer};
pub use room_actor::RoomActor;
use server_state::ServerState;
pub use service_actor::ServiceActorContext;
pub use shutdown_policy::ServiceShutdownPolicy;
use std::time::{Duration, Instant};

#[allow(clippy::unused_async)]
async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("ok"))
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

    /// The method by which new rooms are created and assigned names.
    pub room_id_strategy: RoomIdStrategy,

    /// The port to run the server on. Defaults to 8080.
    pub port: u32,

    /// How the server decides to shut down a room once it is empty.
    pub shutdown_policy: ServiceShutdownPolicy,

    /// A local filesystem path to serve static files from, or None (default).
    pub static_path: Option<String>,

    /// A local filesystem path to serve from /client, or None (default).
    pub client_path: Option<String>,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            heartbeat_interval: Duration::from_secs(30),
            heartbeat_timeout: Duration::from_secs(300),
            port: 8080,
            room_id_strategy: RoomIdStrategy::default(),
            shutdown_policy: ServiceShutdownPolicy::Never,
            static_path: None,
            client_path: None,
        }
    }
}

impl Server {
    #[must_use]
    pub fn new() -> Self {
        Server::default()
    }

    #[cfg(feature="serve-static")]
    #[must_use]
    pub fn with_static_path(mut self, static_path: Option<String>) -> Self {
        self.static_path = static_path;
        self
    }

    #[cfg(feature="serve-static")]
    #[must_use]
    pub fn with_client_path(mut self, client_path: Option<String>) -> Self {
        self.client_path = client_path;
        self
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
    pub fn with_room_id_strategy(mut self, room_id_strategy: RoomIdStrategy) -> Self {
        self.room_id_strategy = room_id_strategy;
        self
    }

    #[must_use]
    pub fn with_shutdown_policy(mut self, shutdown_policy: ServiceShutdownPolicy) -> Self {
        self.shutdown_policy = shutdown_policy;
        self
    }

    pub fn serve_default<F: SimpleJamsocketService>(self) -> std::io::Result<()> {
        let host_factory: SimpleJamsocketServiceFactory<F, ServiceActorContext> =
            SimpleJamsocketServiceFactory::default();

        self.serve(host_factory)
    }

    /// Start a server given a cloneable [JamsocketServiceBuilder].
    ///
    /// This function blocks until the server is terminated. While it is running, the following
    /// endpoints are available:
    /// - `/` (GET): return HTTP 200 if the server is running (useful as a baseline status check)
    /// - `/new_room` (POST): create a new room, if not in `explicit` room creation mode.
    /// - `/ws/{room_id}` (GET): initiate a WebSocket connection to the given room. If the room
    ///     does not exist and the server is in `implicit` room creation mode, it will be created.
    pub fn serve<F: JamsocketServiceFactory<ServiceActorContext>>(
        self,
        host_factory: F,
    ) -> std::io::Result<()> {
        let host = format!("127.0.0.1:{}", self.port);
        let server_state = Data::new(ServerState::new(host_factory, self));

        actix_web::rt::System::new().block_on(async move {
            let server = HttpServer::new(move || {
                #[allow(unused_mut)] // mut only needed with crate feature `serve-static`.
                let mut app = App::new()
                    .app_data(server_state.clone())
                    .route("/status", get().to(status))
                    .route("/new_room", post().to(new_room::<F>))
                    .route("/ws/{room_id}", get().to(websocket::<F>))
                    .route("/ws/{room_id}", post().to(new_room_explicit::<F>));

                #[cfg(feature="serve-static")]
                {
                    if let Some(client_path) = &server_state.settings.client_path {
                        //let client_dir = Path::new(client_path).parent().unwrap();
                        app = app.service(actix_files::Files::new("/client", client_path));
                    }
    
                    if let Some(static_path) = &server_state.settings.static_path {
                        app = app.service(
                            actix_files::Files::new("/", static_path).index_file("index.html"),
                        );
                    }    
                }

                app
            })
            .bind(&host)?;

            log::info!("Listening at {}", &host);
            server.run().await
        })
    }
}

async fn new_room<F: JamsocketServiceFactory<ServiceActorContext>>(
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let server_state: &Data<ServerState<F>> = req.app_data().expect("Could not load ServerState.");
    let room_id = server_state.new_room_generated().await?;

    Ok(HttpResponse::Ok().body(room_id))
}

async fn new_room_explicit<F: JamsocketServiceFactory<ServiceActorContext>>(
    req: HttpRequest,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let server_state: &Data<ServerState<F>> = req.app_data().expect("Could not load ServerState.");
    server_state.explicit_new_room(room_id.as_ref()).await?;

    Ok(HttpResponse::Ok().body(room_id.to_string()))
}

async fn websocket<F: JamsocketServiceFactory<ServiceActorContext>>(
    req: HttpRequest,
    stream: web::Payload,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let ip = req
        .peer_addr()
        .map_or_else(|| "<unknown>".to_string(), |a| a.ip().to_string());

    let server_state: &Data<ServerState<F>> = req.app_data().expect("Could not load ServerState.");
    let room_addr = server_state.connect_room(room_id.as_ref()).await?;

    let client_id = room_addr
        .send(AssignClientId)
        .await
        .map_err(|_| ErrorInternalServerError("Error getting room."))?;

    match ws::start_with_addr(
        ClientSocketConnection {
            room: room_addr.clone().recipient(),
            client_id,
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
                "New connection from IP {} to room {} (user {:?})",
                &ip,
                &room_id,
                client_id
            );
            room_addr.do_send(MessageFromClient::Connect(client_id, addr.recipient()));

            Ok(resp)
        }
        Err(e) => Err(e),
    }
}
