use std::sync::Arc;

use pumpkin::{
    entity::EntityBase,
    plugin::{
        BoxFuture, EventHandler,
        block::block_break::BlockBreakEvent,
        player::player_interact_event::{InteractAction, PlayerInteractEvent},
    },
    server::Server,
};
use pumpkin_data::item::Item;
use pumpkin_util::text::TextComponent;

use crate::storage::WorldEditDataStorage;

pub struct BlockBreakHandler;

impl EventHandler<BlockBreakEvent> for BlockBreakHandler {
    fn handle_blocking<'a>(
        &'a self,
        _server: &'a Arc<Server>,
        event: &'a mut BlockBreakEvent,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let Some(player) = &event.player else { return };
            let item = player.inventory().held_item().lock().await.item;
            if item == &Item::WOODEN_AXE {
                event.cancelled = true;
            }
        })
    }
}

pub struct PlayerInteractHandler {
    storage: Arc<WorldEditDataStorage>,
}

impl PlayerInteractHandler {
    pub fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl EventHandler<PlayerInteractEvent> for PlayerInteractHandler {
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
