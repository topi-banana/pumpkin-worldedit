use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use pumpkin::command::dispatcher::CommandError;
use pumpkin_util::{
    math::{position::BlockPos, vector3::Vector3},
    text::TextComponent,
};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, Default)]
pub struct Selection {
    pos1: Option<BlockPos>,
    pos2: Option<BlockPos>,
}
impl Selection {
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

pub struct SectionStorage {
    sections: Mutex<HashMap<uuid::Uuid, Selection>>,
}

impl SectionStorage {
    pub fn new() -> Self {
        Self {
            sections: Mutex::new(HashMap::new()),
        }
    }
    pub async fn get_selection(
        &self,
        player_uuid: &uuid::Uuid,
    ) -> Result<(BlockPos, BlockPos), CommandError> {
        let section = {
            let sections = self.sections.lock().await;
            sections.get(player_uuid).copied()
        };
        if let Some(selection) = section
            && let Some((mut p1, mut p2)) = selection.get()
        {
            normalization_selection(&mut p1.0, &mut p2.0);
            Ok((p1, p2))
        } else {
            Err(CommandError::CommandFailed(TextComponent::text(
                "Make a region selection first.",
            )))
        }
    }
    pub async fn set_pos1(&self, player_uuid: uuid::Uuid, pos: BlockPos) {
        self.sections
            .lock()
            .await
            .entry(player_uuid)
            .or_default()
            .set_pos1(pos);
    }
    pub async fn set_pos2(&self, player_uuid: uuid::Uuid, pos: BlockPos) {
        self.sections
            .lock()
            .await
            .entry(player_uuid)
            .or_default()
            .set_pos2(pos);
    }
    pub async fn remove_section(&self, player_uuid: &uuid::Uuid) {
        self.sections.lock().await.remove(player_uuid);
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

pub struct Diff {
    pub position: BlockPos,
    pub before: u16,
    // pub after: u16,
}

pub struct History {
    prev: VecDeque<Arc<[Diff]>>,
    next: VecDeque<Arc<[Diff]>>,
}

impl History {
    pub fn new() -> Self {
        Self {
            prev: VecDeque::new(),
            next: VecDeque::new(),
        }
    }
    pub fn push(&mut self, diff: Arc<[Diff]>) {
        self.prev.push_back(diff);
        self.next.clear();
    }
}

pub struct HistoryStorage {
    histories: Mutex<HashMap<uuid::Uuid, History>>,
}

impl HistoryStorage {
    pub fn new() -> Self {
        Self {
            histories: Mutex::new(HashMap::new()),
        }
    }

    pub async fn push(&self, player_uuid: uuid::Uuid, diff: Arc<[Diff]>) {
        self.histories
            .lock()
            .await
            .entry(player_uuid)
            .or_insert(History::new())
            .push(diff);
    }

    pub async fn undo(&self, player_uuid: uuid::Uuid) -> Option<Arc<[Diff]>> {
        let mut histories = self.histories.lock().await;
        if let Some(history) = histories.get_mut(&player_uuid)
            && let Some(diff) = history.prev.pop_back()
        {
            history.next.push_front(diff.clone());
            return Some(diff);
        }
        None
    }
}

pub struct WorldEditDataStorage {
    pub sections: SectionStorage,
    pub histories: HistoryStorage,
}

impl WorldEditDataStorage {
    pub fn new() -> Self {
        Self {
            sections: SectionStorage::new(),
            histories: HistoryStorage::new(),
        }
    }
}
