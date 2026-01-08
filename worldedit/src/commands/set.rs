use std::sync::Arc;

use pumpkin::{
    command::{
        CommandExecutor, CommandResult, CommandSender,
        args::{ConsumedArgs, FindArg, block::BlockArgumentConsumer},
        dispatcher::CommandError,
        tree::{CommandTree, builder::argument},
    },
    entity::EntityBase,
    server::Server,
};
use pumpkin_protocol::java::client::play::{CBlockUpdate, CMultiBlockUpdate};
use pumpkin_util::{
    math::{position::BlockPos, vector2::Vector2, vector3::Vector3},
    text::TextComponent,
};

use crate::{
    storage::{Diff, WorldEditDataStorage},
    utils::chunked_range::ChunkedRange,
};

const NAMES: [&str; 1] = ["/set"];

const DESCRIPTION: &str = "Sets all the blocks in the region";

const ARG_DESC: &str = "The pattern of blocks to set";

struct SetExecuter {
    storage: Arc<WorldEditDataStorage>,
}

impl SetExecuter {
    fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl CommandExecutor for SetExecuter {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _server: &'a Server,
        args: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            let Some(player) = sender.as_player() else {
                return Err(CommandError::PermissionDenied);
            };

            let block_state = BlockArgumentConsumer::find_arg(args, ARG_DESC)?.default_state;

            let player_uuid = player.get_entity().entity_uuid;
            let (pos1, pos2) = self.storage.sections.get_selection(&player_uuid).await?;

            let world = player.world();

            let min_y = world.dimension.min_y;

            let (x1, x2) = (pos1.0.x, pos2.0.x);
            let (z1, z2) = (pos1.0.z, pos2.0.z);
            let (y1, y2) = (pos1.0.y - min_y, pos2.0.y - min_y);

            let mut block_diff = Vec::new();

            /*
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        let block_position = BlockPos(Vector3 { x, y, z });
                        if world.get_block_state(&block_position).await.id != block.id {
                            world
                                .set_block_state(&block_position, block.id, BlockFlags::FORCE_STATE)
                                .await;
                        }
                    }
                }
            }
            */
            // This implementation is a faster version of 👆
            for (chunk_x, x_range) in ChunkedRange::new(x1..=x2) {
                for (chunk_z, z_range) in ChunkedRange::new(z1..=z2) {
                    let chunk = world.level.get_chunk(Vector2::new(chunk_x, chunk_z)).await;
                    let mut chunk = chunk.write().await;
                    for (chunk_y, y_range) in ChunkedRange::new(y1..=y2) {
                        let mut chunk_section = Vec::new();
                        if let Some(section) = chunk.section.sections.get_mut(chunk_y as usize) {
                            for x in x_range.clone() {
                                for z in z_range.clone() {
                                    for y in y_range.clone() {
                                        let cur_block_id = section
                                            .block_states
                                            .get(x as usize, y as usize, z as usize);
                                        if cur_block_id != block_state.id {
                                            section.block_states.set(
                                                x as usize,
                                                y as usize,
                                                z as usize,
                                                block_state.id,
                                            );
                                            let block_pos = BlockPos(Vector3::new(
                                                (chunk_x << 4) + x,
                                                (chunk_y << 4) + y + min_y,
                                                (chunk_z << 4) + z,
                                            ));
                                            block_diff.push(Diff {
                                                position: block_pos,
                                                before: cur_block_id,
                                                after: block_state.id,
                                            });
                                            chunk_section.push((block_pos, block_state.id));
                                        }
                                    }
                                }
                            }
                        }
                        if !chunk_section.is_empty() {
                            chunk.dirty = true;
                            if chunk_section.len() == 1 {
                                let (block_pos, block_state_id) = chunk_section[0];
                                world
                                    .broadcast_packet_all(&CBlockUpdate::new(
                                        block_pos,
                                        i32::from(block_state_id).into(),
                                    ))
                                    .await;
                            } else if !chunk_section.is_empty() {
                                world
                                    .broadcast_packet_all(&CMultiBlockUpdate::new(chunk_section))
                                    .await;
                            }
                        }
                    }
                }
            }

            sender
                .send_message(TextComponent::text(format!(
                    "{} blocks have been changed.",
                    block_diff.len(),
                )))
                .await;

            self.storage
                .histories
                .push(player_uuid, Arc::from(block_diff.into_boxed_slice()))
                .await;

            Ok(())
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_DESC, BlockArgumentConsumer).execute(SetExecuter::new(storage)))
}
