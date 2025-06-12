use std::{collections::HashMap, sync::OnceLock};

use pumpkin::{command::dispatcher::CommandError, plugin::Context};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::math::{position::BlockPos, vector3::Vector3};
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

#[plugin_method]
async fn on_load(&mut self, context: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    log::debug!("Registering commands...");
    commands::register_permission(context).await;
    commands::register_commmand(context).await;
    log::debug!("Commands registered!");

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
pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}
