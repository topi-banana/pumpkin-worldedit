use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use async_trait::async_trait;
use pumpkin::{
    command::dispatcher::CommandError,
    entity::EntityBase,
    plugin::{
        Context, EventHandler, EventPriority,
        player::player_interact_event::{InteractAction, PlayerInteractEvent},
    },
    server::Server,
};
use pumpkin_api_macros::{plugin_impl, plugin_method, with_runtime};
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
    if let Some(&selection) = selections.get(player_uuid) {
        if let Some((p1, p2)) = selection.get() {
            Ok((p1, p2))
        } else {
            Err(CommandError::GeneralCommandIssue(
                "Make a region selection first.".to_string(),
            ))
        }
    } else {
        Err(CommandError::GeneralCommandIssue(
            "Make a region selection first.".to_string(),
        ))
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

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerInteractEvent> for WandHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerInteractEvent) {
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
    }
}

#[plugin_method]
async fn on_load(&mut self, context: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    log::debug!("Registering commands...");
    commands::register_permission(context).await;
    commands::register_commmand(context).await;
    log::debug!("Commands registered!");

    context
        .register_event(Arc::new(WandHandler), EventPriority::Lowest, true)
        .await;

    Ok(())
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

#[plugin_impl]
pub struct Worldedit {}

impl Worldedit {
    pub fn new() -> Self {
        Worldedit {}
    }
}

impl Default for Worldedit {
    fn default() -> Self {
        Self::new()
    }
}
