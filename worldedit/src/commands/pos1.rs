use std::sync::Arc;

use pumpkin::{
    command::{
        CommandExecutor, CommandResult, CommandSender,
        args::{Arg, ConsumedArgs, position_block::BlockPosArgumentConsumer},
        dispatcher::CommandError,
        tree::{CommandTree, builder::argument},
    },
    entity::EntityBase,
    server::Server,
};
use pumpkin_util::{math::position::BlockPos, text::TextComponent};

use crate::storage::WorldEditDataStorage;

const NAMES: [&str; 1] = ["/pos1"];

const DESCRIPTION: &str = "Set position 1";

const ARG_DESC: &str = "Coordinates to set position 1 to";

struct Pos1Executer {
    storage: Arc<WorldEditDataStorage>,
}

impl Pos1Executer {
    fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl CommandExecutor for Pos1Executer {
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
            let block_pos = if let Some(Arg::BlockPos(block_pos)) = args.get(ARG_DESC) {
                *block_pos
            } else {
                BlockPos(player.position().to_i32())
            };

            let message = format!("Started new selection with vertex {}.", block_pos);
            sender.send_message(TextComponent::text(message)).await;

            let player_uuid = player.get_entity().entity_uuid;

            self.storage.sections.set_pos1(player_uuid, block_pos).await;

            Ok(())
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_DESC, BlockPosArgumentConsumer).execute(Pos1Executer::new(storage)))
        .execute(Pos1Executer::new(storage))
}
