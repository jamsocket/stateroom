use crate::service_actor::ServiceActor;
use crate::{RoomActor, Server};
use actix::dev::channel::channel;
use actix::{Addr, Arbiter, Context};
use actix_web::Result;
use stateroom::Stateroom;

const MAILBOX_SIZE: usize = 16;

pub struct ServerState {
    pub room_addr: Addr<RoomActor>,
    pub settings: Server,
}

impl ServerState {
    pub fn new<J>(stateroom: J, settings: Server) -> Result<Self>
    where
        J: Stateroom + Send,
    {
        let arbiter = Arbiter::new();
        let (room_tx, room_rx) = channel(MAILBOX_SIZE);
        let (service_tx, service_rx) = channel(MAILBOX_SIZE);
        let room_addr = Addr::new(room_tx);
        let service_addr = Addr::new(service_tx);

        {
            let room_addr = room_addr.clone();

            arbiter.spawn_fn(move || {
                let room_ctx = Context::with_receiver(room_rx);
                let service_ctx = Context::with_receiver(service_rx);

                let service_actor = ServiceActor::new(stateroom, room_addr.clone().recipient());

                let room_actor = RoomActor::new(service_addr.recipient());

                room_ctx.run(room_actor);
                if let Some(service_actor) = service_actor {
                    service_ctx.run(service_actor);
                } else {
                    tracing::error!("Could not create service actor for room");
                }
            });
        }

        Ok(ServerState {
            settings,
            room_addr,
        })
    }
}
