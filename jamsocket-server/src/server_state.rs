pub use crate::room_id::{RoomIdGenerator, RoomIdStrategy, UuidRoomIdGenerator};
use crate::service_actor::{ServiceActor, ServiceActorContext};
use crate::{RoomActor, ServerSettings};
use actix::{Actor, Addr, AsyncContext};
use actix_web::error::{ErrorBadRequest, ErrorConflict, ErrorInternalServerError, ErrorNotFound};
use actix_web::Result;
use async_std::sync::{Mutex, RwLock};
use jamsocket::JamsocketServiceBuilder;
use std::collections::HashMap;

pub struct ServerState<T: JamsocketServiceBuilder<ServiceActorContext> + Clone> {
    mapping: RwLock<HashMap<String, Addr<RoomActor>>>,
    generator: Option<Mutex<Box<dyn RoomIdGenerator>>>,
    pub settings: ServerSettings,
    host_factory: T,
}

impl<T: JamsocketServiceBuilder<ServiceActorContext> + Clone> ServerState<T> {
    pub fn new(host_factory: T, settings: ServerSettings) -> Self {
        ServerState {
            mapping: Default::default(),
            generator: settings.room_id_strategy.try_generator().map(Mutex::new),
            settings,
            host_factory,
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
                let host_factory = self.host_factory.clone();

                let room_actor = {
                    RoomActor::create(|room_actor_context| {
                        let service_actor = ServiceActor::create(|service_actor_context| {
                            ServiceActor::new(
                                service_actor_context,
                                room_id.to_string(),
                                host_factory,
                                room_actor_context.address().recipient(),
                            )
                            .unwrap()
                        });

                        RoomActor::new(room_id.to_string(), service_actor.recipient())
                    })
                };

                entry.insert(room_actor.clone());

                log::info!("Created room: {}", &room_id);
                Ok(room_actor)
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
                log::error!("Ran out of unique room IDs. Increase the length to avoid this.");
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
