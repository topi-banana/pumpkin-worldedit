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
use pumpkin_util::{
    math::{position::BlockPos, vector3::Vector3},
    text::TextComponent,
};
use pumpkin_world::world::BlockFlags;

use crate::storage::{Diff, WorldEditDataStorage};

const NAMES: [&str; 3] = ["/replace", "/re", "/rep"];

const DESCRIPTION: &str = "Replace all blocks in the selection with another";

const ARG_DESC_FROM: &str = "The mask representing blocks to replace";
const ARG_DESC_TO: &str = "The pattern of blocks to set";

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

            let block_from = BlockArgumentConsumer::find_arg(args, ARG_DESC_FROM)?.default_state;
            let block_to = BlockArgumentConsumer::find_arg(args, ARG_DESC_TO)?.default_state;

            let player_uuid = player.get_entity().entity_uuid;
            let (pos1, pos2) = self.storage.sections.get_selection(&player_uuid).await?;

            let world = player.world();

            let (x1, x2) = (pos1.0.x, pos2.0.x);
            let (z1, z2) = (pos1.0.z, pos2.0.z);
            let (y1, y2) = (pos1.0.y, pos2.0.y);

            let mut block_diff = Vec::new();

            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        let block_position = BlockPos(Vector3 { x, y, z });
                        if world.get_block_state(&block_position).await.id == block_from.id {
                            block_diff.push(Diff {
                                position: block_position,
                                before: block_from.id,
                                after: block_to.id,
                            });
                            world
                                .set_block_state(
                                    &block_position,
                                    block_to.id,
                                    BlockFlags::FORCE_STATE,
                                )
                                .await;
                        }
                    }
                }
            }

            sender
                .send_message(TextComponent::text(format!(
                    "{} blocks have been changed.",
                    block_diff.len()
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
    CommandTree::new(NAMES, DESCRIPTION).then(
        argument(ARG_DESC_FROM, BlockArgumentConsumer)
            .then(argument(ARG_DESC_TO, BlockArgumentConsumer).execute(SetExecuter::new(storage))),
    )
}
