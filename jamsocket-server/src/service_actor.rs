use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Recipient, SpawnHandle};
use anyhow::Result;
use jamsocket::{JamsocketContext, JamsocketService, JamsocketServiceBuilder, MessageRecipient};
use std::time::Duration;

use crate::{
    messages::{MessageFromClient, MessageFromServer},
    RoomActor,
};

pub struct ServiceActor<T: JamsocketService + Unpin> {
    service: T,
    timer_handle: Option<SpawnHandle>,
    room: Addr<RoomActor>,
}

struct SetTimer(u32);

impl Message for SetTimer {
    type Result = ();
}
struct TimerFinished;

impl Message for TimerFinished {
    type Result = ();
}

pub struct GetRoomAddr;

impl Message for GetRoomAddr {
    type Result = Addr<RoomActor>;
}

impl<T: JamsocketService + 'static + Unpin> Handler<GetRoomAddr> for ServiceActor<T> {
    type Result = Addr<RoomActor>;

    fn handle(&mut self, _: GetRoomAddr, _: &mut Self::Context) -> Self::Result {
        self.room.clone()
    }
}

pub struct ServiceActorContext {
    set_timer_recipient: Recipient<SetTimer>,
    send_message_recipient: Recipient<MessageFromServer>,
}

impl JamsocketContext for ServiceActorContext {
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str) {
        self.send_message_recipient
            .do_send(MessageFromServer::new(
                recipient.into(),
                message.to_string(),
            ))
            .unwrap();
    }

    fn set_timer(&self, ms_delay: u32) {
        self.set_timer_recipient
            .do_send(SetTimer(ms_delay))
            .unwrap();
    }
}

impl<T: JamsocketService + 'static + Unpin> ServiceActor<T> {
    pub fn new(
        ctx: &mut Context<Self>,
        token: &str,
        service_constructor: impl JamsocketServiceBuilder<ServiceActorContext, Service=T>,
    ) -> Result<Self> {
        let room_actor = RoomActor::new(ctx.address().recipient()).start();

        let recipient = room_actor.clone().recipient();

        let host_context = ServiceActorContext {
            set_timer_recipient: ctx.address().recipient(),
            send_message_recipient: recipient,
        };
        let service = service_constructor.build(token, host_context);

        Ok(ServiceActor {
            service,
            timer_handle: None,
            room: room_actor,
        })
    }
}

impl<T: JamsocketService + 'static + Unpin> Actor for ServiceActor<T> {
    type Context = Context<Self>;
}

impl<T: JamsocketService + 'static + Unpin> Handler<MessageFromClient> for ServiceActor<T> {
    type Result = ();

    fn handle(&mut self, msg: MessageFromClient, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            MessageFromClient::Connect(u, _) => {
                self.service.connect(u);
            }
            MessageFromClient::Disconnect(u) => {
                self.service.disconnect(u);
            }
            MessageFromClient::Message { data, from_user } => {
                self.service.message(from_user, &data);
            }
        }
    }
}

impl<T: JamsocketService + 'static + Unpin> Handler<SetTimer> for ServiceActor<T> {
    type Result = ();

    fn handle(&mut self, SetTimer(duration_ms): SetTimer, ctx: &mut Self::Context) -> Self::Result {
        if let Some(timer_handle) = self.timer_handle {
            ctx.cancel_future(timer_handle);
        }

        let handle = ctx.notify_later(TimerFinished, Duration::from_millis(duration_ms as u64));

        self.timer_handle = Some(handle);
    }
}

impl<T: JamsocketService + 'static + Unpin> Handler<TimerFinished> for ServiceActor<T> {
    type Result = ();

    fn handle(&mut self, _: TimerFinished, _: &mut Self::Context) -> Self::Result {
        self.service.timer();
    }
}
