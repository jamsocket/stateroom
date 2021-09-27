use crate::messages::{MessageData, MessageFromClient, MessageFromServer};
use actix::{
    prelude::SendError, Actor, AsyncContext, Context, Handler, Message, Recipient, SpawnHandle,
};
use jamsocket::{JamsocketContext, JamsocketService, JamsocketServiceFactory, MessageRecipient};
use std::{sync::Arc, time::Duration};

pub struct ServiceActor<J: JamsocketService> {
    service: J,
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

/// A [JamsocketContext] implementation for [JamsocketService]s hosted in the
/// context of a [ServiceActor].
#[derive(Clone)]
pub struct ServiceActorContext {
    set_timer_recipient: Recipient<SetTimer>,
    send_message_recipient: Recipient<MessageFromServer>,
}

impl ServiceActorContext {
    fn try_send(&self, message: MessageFromServer) {
        match self.send_message_recipient.do_send(message) {
            Ok(_) => (),
            Err(SendError::Closed(_)) => {
                tracing::warn!("Attempted to send a message to a closed service");
            }
            Err(error) => {
                tracing::warn_span!("Encountered an error sending message to a service", ?error);
            }
        }
    }
}

impl JamsocketContext for ServiceActorContext {
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str) {
        self.try_send(MessageFromServer::new(
            recipient.into(),
            message.to_string(),
        ));
    }

    fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]) {
        self.try_send(MessageFromServer::new_binary(
            recipient.into(),
            message.to_vec(),
        ));
    }

    fn set_timer(&self, ms_delay: u32) {
        if self
            .set_timer_recipient
            .do_send(SetTimer(ms_delay))
            .is_err()
        {
            tracing::warn!("Encountered an error sending SetTimer message.");
        }
    }
}

impl<J: JamsocketService> ServiceActor<J> {
    pub fn new(
        ctx: &Context<Self>,
        room_id: &str,
        service_constructor: Arc<impl JamsocketServiceFactory<ServiceActorContext, Service = J>>,
        recipient: Recipient<MessageFromServer>,
    ) -> Option<Self> {
        let host_context = ServiceActorContext {
            set_timer_recipient: ctx.address().recipient(),
            send_message_recipient: recipient,
        };
        let service = service_constructor.build(room_id, host_context)?;

        Some(ServiceActor {
            service,
            timer_handle: None,
        })
    }
}

impl<J: JamsocketService> Actor for ServiceActor<J> {
    type Context = Context<Self>;

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        tracing::info!("Shutting down service");
        actix::Running::Stop
    }
}

impl<J: JamsocketService> Handler<MessageFromClient> for ServiceActor<J> {
    type Result = ();

    fn handle(&mut self, msg: MessageFromClient, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            MessageFromClient::Connect(u, _) => {
                self.service.connect(u);
            }
            MessageFromClient::Disconnect(u) => {
                self.service.disconnect(u);
            }
            MessageFromClient::Message { data, from_client } => match data {
                MessageData::Binary(bin) => self.service.binary(from_client, &bin),
                MessageData::String(st) => self.service.message(from_client, &st),
            },
        }
    }
}

impl<J: JamsocketService> Handler<SetTimer> for ServiceActor<J> {
    type Result = ();

    fn handle(&mut self, SetTimer(duration_ms): SetTimer, ctx: &mut Self::Context) -> Self::Result {
        tracing::info_span!("Timer set", %duration_ms);

        if let Some(timer_handle) = self.timer_handle.take() {
            ctx.cancel_future(timer_handle);
        }

        if duration_ms > 0 {
            let handle =
                ctx.notify_later(TimerFinished, Duration::from_millis(u64::from(duration_ms)));
            self.timer_handle = Some(handle);
        }
    }
}

impl<J: JamsocketService> Handler<TimerFinished> for ServiceActor<J> {
    type Result = ();

    fn handle(&mut self, _: TimerFinished, _: &mut Self::Context) -> Self::Result {
        tracing::info!("Timer finished.");
        self.service.timer();
    }
}
