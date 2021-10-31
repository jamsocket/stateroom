use jamsocket::*;
use jamsocket_server::*;

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
    Server::new().serve(EchoServer)?;

    Ok(())
}
