use async_trait::async_trait;
use pumpkin::command::CommandExecutor;
use pumpkin::command::CommandSender;
use pumpkin::command::args::ConsumedArgs;
use pumpkin::command::args::FindArg;
use pumpkin::command::args::block::BlockArgumentConsumer;
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::tree::CommandTree;
use pumpkin::command::tree::builder::argument;
use pumpkin::entity::EntityBase;
use pumpkin::server::Server;
use pumpkin_protocol::client::play::CBlockUpdate;
use pumpkin_protocol::client::play::CMultiBlockUpdate;
use pumpkin_registry::DimensionType;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector2::Vector2;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_util::text::TextComponent;

use crate::utils::vector_range::ChunkSplitRange;

const NAMES: [&str; 1] = ["/set"];

const DESCRIPTION: &str = "Sets all the blocks in the region";

const ARG_DESC: &str = "The pattern of blocks to set";

struct SetExecuter;

#[async_trait]
impl CommandExecutor for SetExecuter {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(player) = sender.as_player() else {
            return Err(CommandError::PermissionDenied);
        };

        let block = BlockArgumentConsumer::find_arg(args, ARG_DESC)?;

        let (mut pos1, mut pos2) =
            crate::fetch_selections(&player.get_entity().entity_uuid).await?;

        crate::normalization_selection(&mut pos1.0, &mut pos2.0);

        let world = player.world().await;

        let min_y = match world.dimension_type {
            DimensionType::Overworld | DimensionType::OverworldCaves => -64,
            DimensionType::TheEnd | DimensionType::TheNether => 0,
        };

        let (x1, x2) = (pos1.0.x, pos2.0.x);
        let (z1, z2) = (pos1.0.z, pos2.0.z);
        let (y1, y2) = (pos1.0.y - min_y, pos2.0.y - min_y);

        let mut total_cnt = 0;

        /*
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    let block_position = BlockPos(Vector3 { x, y, z });
                    if world.get_block_state(&block_position).await.id != block.id {
                        world
                            .set_block_state(&block_position, block.id, BlockFlags::FORCE_STATE)
                            .await;
                        total_cnt += 1;
                    }
                }
            }
        }
        */
        // This implementation is a faster version of 👆
        for (chunk_x, x_range) in ChunkSplitRange::new(x1..=x2) {
            for (chunk_z, z_range) in ChunkSplitRange::new(z1..=z2) {
                let chunk = world.level.get_chunk(Vector2::new(chunk_x, chunk_z)).await;
                let mut chunk = chunk.write().await;
                let mut cnt = 0;
                for (chunk_y, y_range) in ChunkSplitRange::new(y1..=y2) {
                    let mut chunk_section = Vec::new();
                    if let Some(section) = chunk.section.sections.get_mut(chunk_y as usize) {
                        for x in x_range.clone() {
                            for z in z_range.clone() {
                                for y in y_range.clone() {
                                    let cur_block_id = section
                                        .block_states
                                        .get(x as usize, y as usize, z as usize);
                                    if cur_block_id != block.id {
                                        section
                                            .block_states
                                            .set(x as usize, y as usize, z as usize, block.id);
                                        chunk_section.push((
                                            BlockPos(Vector3::new(
                                                (chunk_x << 4) + x,
                                                (chunk_y << 4) + y + min_y,
                                                (chunk_z << 4) + z,
                                            )),
                                            block.id,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    if !chunk_section.is_empty() {
                        cnt += chunk_section.len();
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
                if cnt != 0 {
                    chunk.dirty = true;
                    total_cnt += cnt;
                }
                drop(chunk);
            }
        }

        sender
            .send_message(TextComponent::text(format!(
                "{} blocks have been changed.",
                total_cnt,
            )))
            .await;

        Ok(())
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_DESC, BlockArgumentConsumer).execute(SetExecuter))
}
