use stateroom::*;
use stateroom_server::*;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct EchoServer;

impl SimpleStateroomService for EchoServer {
    fn new(_: &str, _: &impl StateroomContext) -> Self {
        EchoServer
    }

    fn message(&mut self, client: ClientId, message: &str, ctx: &impl StateroomContext) {
        ctx.send_message(client, &format!("echo: {}", message));
    }
}

fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::default().add_directive("info".parse().unwrap());

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    Server::new().serve(EchoServer)?;

    Ok(())
}
