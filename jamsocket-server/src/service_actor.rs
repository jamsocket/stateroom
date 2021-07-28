use crate::{
    messages::{MessageData, MessageFromClient, MessageFromServer},
};
use actix::{Actor, AsyncContext, Context, Handler, Message, Recipient, SpawnHandle};
use anyhow::Result;
use jamsocket::{JamsocketContext, JamsocketService, JamsocketServiceBuilder, MessageRecipient};
use std::time::Duration;

pub struct ServiceActor<T: JamsocketService> {
    service: T,
    timer_handle: Option<SpawnHandle>,
}

struct SetTimer(u32);

impl Message for SetTimer {
    type Result = ();
}
struct TimerFinished;

impl Message for TimerFinished {
    type Result = ();
}

pub struct ServiceActorContext {
    set_timer_recipient: Recipient<SetTimer>,
    send_message_recipient: Recipient<MessageFromServer>,
}

impl JamsocketContext for ServiceActorContext {
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str) {
        println!("message: {}", message);
        self.send_message_recipient
            .do_send(MessageFromServer::new(
                recipient.into(),
                message.to_string(),
            ))
            .unwrap();
    }

    fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]) {
        self.send_message_recipient
            .do_send(MessageFromServer::new_binary(
                recipient.into(),
                message.to_vec(),
            ))
            .unwrap();
    }

    fn set_timer(&self, ms_delay: u32) {
        self.set_timer_recipient
            .do_send(SetTimer(ms_delay))
            .unwrap();
    }
}

impl<T: JamsocketService + Unpin + 'static> ServiceActor<T> {
    pub fn new(
        ctx: &mut Context<Self>,
        room_id: String,
        service_constructor: impl JamsocketServiceBuilder<ServiceActorContext, Service = T>,
        recipient: Recipient<MessageFromServer>,
    ) -> Result<Self> {
        let host_context = ServiceActorContext {
            set_timer_recipient: ctx.address().recipient(),
            send_message_recipient: recipient,
        };
        let service = service_constructor.build(&room_id, host_context);

        Ok(ServiceActor {
            service,
            timer_handle: None,
        })
    }
}

impl<T: JamsocketService + 'static + Unpin> Actor for ServiceActor<T> {
    type Context = Context<Self>;

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        log::info!("Shutting down service.");
        actix::Running::Stop
    }
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
            MessageFromClient::Message { data, from_user } => match data {
                MessageData::Binary(bin) => self.service.binary(from_user, &bin),
                MessageData::String(st) => self.service.message(from_user, &st),
            },
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
