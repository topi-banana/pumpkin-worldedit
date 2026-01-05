use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use pumpkin::{
    command::dispatcher::CommandError,
    entity::EntityBase,
    plugin::{
        BoxFuture, Context, EventHandler, EventPriority, Plugin, PluginFuture, PluginMetadata,
        player::player_interact_event::{InteractAction, PlayerInteractEvent},
    },
    server::Server,
};
use pumpkin_data::item::Item;
use pumpkin_util::{
    math::{position::BlockPos, vector3::Vector3},
    text::TextComponent,
};
use tokio::sync::RwLock;

pub mod utils;

mod commands;

type Selections = HashMap<uuid::Uuid, Selection>;

static SELECTIONS: OnceLock<RwLock<Selections>> = OnceLock::new();

fn selections() -> &'static RwLock<Selections> {
    SELECTIONS.get_or_init(|| RwLock::new(HashMap::new()))
}

async fn fetch_selections(player_uuid: &uuid::Uuid) -> Result<(BlockPos, BlockPos), CommandError> {
    let selections = crate::selections().read().await;
    if let Some(&selection) = selections.get(player_uuid)
        && let Some((p1, p2)) = selection.get()
    {
        Ok((p1, p2))
    } else {
        Err(CommandError::CommandFailed(TextComponent::text(
            "Make a region selection first.",
        )))
    }
}

fn normalization_selection<T: PartialOrd>(pos1: &mut Vector3<T>, pos2: &mut Vector3<T>) {
    if pos1.x > pos2.x {
        std::mem::swap(&mut pos1.x, &mut pos2.x);
    }
    if pos1.y > pos2.y {
        std::mem::swap(&mut pos1.y, &mut pos2.y);
    }
    if pos1.z > pos2.z {
        std::mem::swap(&mut pos1.z, &mut pos2.z);
    }
}

struct WandHandler;

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

            let message = {
                let mut selections = crate::selections().write().await;
                let selection = selections.entry(player_uuid).or_default();
                match event.action {
                    InteractAction::LeftClickBlock => {
                        selection.set_pos1(pos);
                        format!("Started new selection with vertex {}.", pos)
                    }
                    InteractAction::RightClickBlock => {
                        selection.set_pos2(pos);
                        format!("Added vertex {} to the selection.", pos)
                    }
                    _ => return,
                }
            };

            event.cancelled = true;

            event
                .player
                .send_system_message(&TextComponent::text(message))
                .await;
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Selection {
    pos1: Option<BlockPos>,
    pos2: Option<BlockPos>,
}
impl Selection {
    #[must_use]
    pub fn pos1(&self) -> Option<BlockPos> {
        self.pos1
    }
    #[must_use]
    pub fn pos2(&self) -> Option<BlockPos> {
        self.pos2
    }

    pub fn set_pos1(&mut self, pos: BlockPos) {
        self.pos1 = Some(pos);
    }
    pub fn set_pos2(&mut self, pos: BlockPos) {
        self.pos2 = Some(pos);
    }

    #[must_use]
    pub fn get(&self) -> Option<(BlockPos, BlockPos)> {
        if let (Some(pos1), Some(pos2)) = (self.pos1, self.pos2) {
            Some((pos1, pos2))
        } else {
            None
        }
    }
}

pub struct Worldedit;

impl Plugin for Worldedit {
    fn on_load(&mut self, server: Arc<Context>) -> PluginFuture<'_, Result<(), String>> {
        Box::pin(async move {
            log::info!("Hello, Pumpkin!");

            log::debug!("Registering commands...");
            commands::register_permission(&server).await;
            commands::register_command(&server).await;
            log::debug!("Commands registered!");

            server
                .register_event(Arc::new(WandHandler), EventPriority::Lowest, true)
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
    Box::new(Worldedit)
}

#[unsafe(no_mangle)]
pub static METADATA: PluginMetadata = PluginMetadata {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    authors: env!("CARGO_PKG_AUTHORS"),
    description: env!("CARGO_PKG_DESCRIPTION"),
};
