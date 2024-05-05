use crate::server::Event;
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    routing::get,
    Router,
};
use server::ServerState;
use stateroom::StateroomServiceFactory;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::{net::TcpListener, select};
use tower_http::services::ServeDir;

mod server;

const DEFAULT_IP: &str = "0.0.0.0";

#[derive(Debug)]
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
    pub port: u16,

    /// The IP to listen on. Defaults to 0.0.0.0.
    pub ip: String,

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
            ip: DEFAULT_IP.to_string(),
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

    #[must_use]
    pub fn with_static_path(mut self, static_path: Option<String>) -> Self {
        self.static_path = static_path;
        self
    }

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
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    #[must_use]
    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip = ip;
        self
    }

    /// Start a server given a [StateroomService].
    ///
    /// This function blocks until the server is terminated. While it is running, the following
    /// endpoints are available:
    /// - `/` (GET): return HTTP 200 if the server is running (useful as a baseline status check)
    /// - `/ws` (GET): initiate a WebSocket connection to the stateroom service.
    pub async fn serve_async(self, factory: impl StateroomServiceFactory) -> std::io::Result<()> {
        let server_state = Arc::new(ServerState::new(factory));

        let mut app = Router::new()
            .route("/ws", get(serve_websocket))
            .with_state(server_state);

        if let Some(static_path) = self.static_path {
            app = app.nest_service("/", ServeDir::new(static_path));
        }

        if let Some(client_path) = self.client_path {
            app = app.nest_service("/client", ServeDir::new(client_path));
        }

        let ip = self.ip.parse::<IpAddr>().unwrap();
        let addr = SocketAddr::new(ip, self.port);
        let listener = TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Start a server given a [StateroomService].
    ///
    /// This function blocks until the server is terminated. While it is running, the following
    /// endpoints are available:
    /// - `/` (GET): return HTTP 200 if the server is running (useful as a baseline status check)
    /// - `/ws` (GET): initiate a WebSocket connection to the stateroom service.
    pub fn serve(self, factory: impl StateroomServiceFactory) -> std::io::Result<()> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { self.serve_async(factory).await })
    }
}

pub async fn serve_websocket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<ServerState>) {
    let (send, mut recv, client_id) = state.connect();

    loop {
        select! {
            msg = recv.recv() => {
                match msg {
                    Some(msg) => socket.send(msg).await.unwrap(),
                    None => break,
                }
            },
            msg = socket.recv() => {
                match msg {
                    Some(Ok(msg)) => send.send(Event::Message { client: client_id, message: msg }).await.unwrap(),
                    Some(Err(_)) => todo!("Error receiving message from client."),
                    None => break,
                }
            }
        }
    }

    state.remove(&client_id);
}
