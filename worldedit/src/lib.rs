use std::sync::Arc;

use pumpkin::{
    entity::EntityBase,
    plugin::{
        BoxFuture, Context, EventHandler, EventPriority, Plugin, PluginFuture, PluginMetadata,
        player::player_interact_event::{InteractAction, PlayerInteractEvent},
    },
    server::Server,
};
use pumpkin_data::item::Item;
use pumpkin_util::text::TextComponent;

use crate::storage::WorldEditDataStorage;

mod storage;
mod utils;

mod commands;

struct WandHandler {
    storage: Arc<WorldEditDataStorage>,
}

impl WandHandler {
    pub fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl EventHandler<PlayerInteractEvent> for WandHandler {
    fn handle_blocking<'a>(
        &'a self,
        _server: &'a Arc<Server>,
        event: &'a mut PlayerInteractEvent,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let Some(pos) = event.clicked_pos else {
                return;
            };

            if event.item.lock().await.item != &Item::WOODEN_AXE {
                return;
            }

            let player_uuid = event.player.get_entity().entity_uuid;

            let message = match event.action {
                InteractAction::LeftClickBlock => {
                    self.storage.sections.set_pos1(player_uuid, pos).await;
                    format!("Started new selection with vertex {}.", pos)
                }
                InteractAction::RightClickBlock => {
                    self.storage.sections.set_pos2(player_uuid, pos).await;
                    format!("Added vertex {} to the selection.", pos)
                }
                _ => return,
            };

            event.cancelled = true;

            event
                .player
                .send_system_message(&TextComponent::text(message))
                .await;
        })
    }
}

pub struct Worldedit {
    storage: Arc<WorldEditDataStorage>,
}

impl Plugin for Worldedit {
    fn on_load(&mut self, server: Arc<Context>) -> PluginFuture<'_, Result<(), String>> {
        Box::pin(async move {
            log::info!("Hello, Pumpkin!");

            log::debug!("Registering commands...");
            commands::register_permission(&server).await;
            commands::register_command(&server, &self.storage).await;
            log::debug!("Commands registered!");

            server
                .register_event(
                    Arc::new(WandHandler::new(&self.storage)),
                    EventPriority::Highest,
                    true,
                )
                .await;

            Ok(())
        })
    }

    fn on_unload(&mut self, _server: Arc<Context>) -> PluginFuture<'_, Result<(), String>> {
        Box::pin(async { Ok(()) })
    }
}

#[unsafe(no_mangle)]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Worldedit {
        storage: Arc::new(WorldEditDataStorage::new()),
    })
}

#[unsafe(no_mangle)]
pub static METADATA: PluginMetadata = PluginMetadata {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    authors: env!("CARGO_PKG_AUTHORS"),
    description: env!("CARGO_PKG_DESCRIPTION"),
};
