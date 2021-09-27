pub use crate::room_id::{RoomIdGenerator, RoomIdStrategy, UuidRoomIdGenerator};
use crate::service_actor::{ServiceActor, ServiceActorContext};
use crate::{RoomActor, Server};
use actix::dev::channel::channel;
use actix::{Addr, Arbiter, Context};
use actix_web::error::{ErrorBadRequest, ErrorConflict, ErrorInternalServerError, ErrorNotFound};
use actix_web::Result;
use async_std::sync::{Mutex, RwLock};
use jamsocket::JamsocketServiceFactory;
use std::collections::HashMap;
use std::sync::Arc;

const MAILBOX_SIZE: usize = 16;

pub struct ServerState<T: JamsocketServiceFactory<ServiceActorContext>> {
    mapping: RwLock<HashMap<String, Addr<RoomActor>>>,
    generator: Option<Mutex<Box<dyn RoomIdGenerator>>>,
    pub settings: Server,
    host_factory: Arc<T>,
}

impl<T: JamsocketServiceFactory<ServiceActorContext>> ServerState<T> {
    pub fn new(host_factory: T, settings: Server) -> Self {
        ServerState {
            mapping: RwLock::default(),
            generator: settings.room_id_strategy.try_generator().map(Mutex::new),
            settings,
            host_factory: Arc::new(host_factory),
        }
    }

    pub async fn explicit_new_room(&self, room_id: &str) -> Result<Addr<RoomActor>> {
        if !self
            .settings
            .room_id_strategy
            .explicit_room_creation_allowed()
        {
            return Err(ErrorInternalServerError(
                "Explicit room creation is not enabled.",
            ));
        }

        self.new_room(room_id, false).await
    }

    pub async fn new_room(&self, room_id: &str, exists_ok: bool) -> Result<Addr<RoomActor>> {
        match self.mapping.write().await.entry(room_id.to_string()) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                if exists_ok {
                    Ok(entry.get().clone())
                } else {
                    Err(ErrorConflict(
                        "Attempted to create a room that already exists.",
                    ))
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let arbiter = Arbiter::new();
                let (room_tx, room_rx) = channel(MAILBOX_SIZE);
                let (service_tx, service_rx) = channel(MAILBOX_SIZE);
                let room_addr = Addr::new(room_tx);
                let service_addr = Addr::new(service_tx);

                {
                    let room_id = room_id.to_string();
                    let host_factory = self.host_factory.clone();
                    let shutdown_policy = self.settings.shutdown_policy;
                    let room_addr = room_addr.clone();

                    arbiter.spawn_fn(move || {
                        let room_ctx = Context::with_receiver(room_rx);
                        let service_ctx = Context::with_receiver(service_rx);

                        let service_actor = ServiceActor::new(
                            &service_ctx,
                            &room_id,
                            host_factory,
                            room_addr.clone().recipient(),
                        );

                        let room_actor = RoomActor::new(
                            room_id.clone(),
                            service_addr.recipient(),
                            shutdown_policy,
                        );

                        room_ctx.run(room_actor);
                        if let Some(service_actor) = service_actor {
                            service_ctx.run(service_actor);
                        } else {
                            tracing::error_span!("Could not create service actor for room", %room_id);
                        }
                    });
                }

                entry.insert(room_addr.clone());

                tracing::info_span!("Created room", room_id=%room_id);
                Ok(room_addr)
            }
        }
    }

    pub async fn connect_room(&self, room_id: &str) -> Result<Addr<RoomActor>> {
        let maybe_room_addr = { self.mapping.read().await.get(room_id).cloned() };

        if let Some(room_addr) = maybe_room_addr {
            Ok(room_addr)
        } else if self
            .settings
            .room_id_strategy
            .implicit_room_creation_allowed()
        {
            self.new_room(room_id, true).await
        } else {
            Err(ErrorNotFound("The requested room does not exist."))
        }
    }

    pub async fn new_room_generated(&self) -> Result<String> {
        if let Some(generator) = &self.generator {
            let room_id = { generator.lock().await.generate() };
            let result = self.new_room(&room_id, false).await;

            if result.is_err() {
                tracing::error_span!("Ran out of unique room IDs. Increase the length to avoid this.");
                return Err(ErrorConflict("Ran out of unique room IDs."));
            }

            Ok(room_id)
        } else {
            Err(ErrorBadRequest(
                "Room ID generation is disabled; rooms must be created with an explicit ID.",
            ))
        }
    }
}
