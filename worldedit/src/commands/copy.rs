use std::sync::Arc;

use pumpkin::{
    command::{
        CommandExecutor, CommandResult, CommandSender, args::ConsumedArgs,
        dispatcher::CommandError, tree::CommandTree,
    },
    entity::EntityBase,
    server::Server,
};
use pumpkin_util::{
    math::{position::BlockPos, vector2::Vector2, vector3::Vector3},
    text::TextComponent,
};

use crate::{
    storage::{ClipBoard, WorldEditDataStorage},
    utils::chunked_range::ChunkedRange,
};

const NAMES: [&str; 1] = ["/copy"];

const DESCRIPTION: &str = "Copy the selection to the clipboard";

struct CopyExecutor {
    storage: Arc<WorldEditDataStorage>,
}

impl CopyExecutor {
    fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl CommandExecutor for CopyExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _server: &'a Server,
        _args: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            let Some(player) = sender.as_player() else {
                return Err(CommandError::PermissionDenied);
            };
            let world = player.world();
            let player_uuid = player.get_entity().entity_uuid;

            let (pos1, pos2) = self.storage.sections.get_selection(&player_uuid).await?;

            let mut data = Vec::new();

            for (chunk_x, x_range) in ChunkedRange::new(pos1.0.x..=pos2.0.x) {
                for (chunk_z, z_range) in ChunkedRange::new(pos1.0.z..=pos2.0.z) {
                    let chunk = world.level.get_chunk(Vector2::new(chunk_x, chunk_z)).await;
                    let mut chunk = chunk.write().await;
                    for (chunk_y, y_range) in ChunkedRange::new(pos1.0.y..=pos2.0.y) {
                        if let Some(section) = chunk.section.sections.get_mut(chunk_y as usize) {
                            for x in x_range.clone() {
                                for z in z_range.clone() {
                                    for y in y_range.clone() {
                                        let block_id = section
                                            .block_states
                                            .get(x as usize, y as usize, z as usize);

                                        let block_pos = BlockPos(Vector3::new(
                                            (chunk_x << 4) + x,
                                            (chunk_y << 4) + y + world.dimension.min_y,
                                            (chunk_z << 4) + z,
                                        ));

                                        data.push((block_pos, block_id));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let message = TextComponent::text(format!("{} blocks affected", data.len()));

            self.storage
                .clipboard
                .push(player_uuid, ClipBoard::new(data))
                .await;

            sender.send_message(message).await;

            Ok(())
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(CopyExecutor::new(storage))
}
