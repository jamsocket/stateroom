use stateroom::*;
use stateroom_server::*;
use tracing_subscriber::EnvFilter;
use async_trait::async_trait;

#[derive(Clone)]
struct EchoServer;

#[async_trait]
impl Stateroom for EchoServer {
    async fn run<C: StateroomContext>(mut ctx: C) -> () {
        loop {
            let message = ctx.next_message().await;

            match message {
                MessageToRoom::Connect { .. } => {
                    ctx.send(MessageRecipient::Broadcast, "Hello.");
                },
                MessageToRoom::Message { client, message: MessagePayload::Text(text) } => {
                    ctx.send(client, &format!("Got message: {}", text));
                },
                _ => ()
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::default().add_directive("info".parse().unwrap());

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    Server::new().serve::<EchoServer>()?;

    Ok(())
}
