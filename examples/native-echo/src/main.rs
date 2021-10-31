use jamsocket::*;
use jamsocket_server::*;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct EchoServer;

impl SimpleJamsocketService for EchoServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        EchoServer
    }

    fn message(&mut self, client: ClientId, message: &str, ctx: &impl JamsocketContext) {
        ctx.send_message(client, &format!("echo: {}", message));
    }
}

fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::default().add_directive("info".parse().unwrap());

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    Server::new().serve(EchoServer)?;

    Ok(())
}
