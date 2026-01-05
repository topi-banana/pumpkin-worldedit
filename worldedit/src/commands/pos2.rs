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

const NAMES: [&str; 1] = ["/pos2"];

const DESCRIPTION: &str = "Set position 2";

const ARG_DESC: &str = "Coordinates to set position 2 to";

struct Pos2Executer;

impl CommandExecutor for Pos2Executer {
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

            let message = format!("Added vertex {} to the selection.", block_pos);
            sender.send_message(TextComponent::text(message)).await;

            let player_uuid = player.get_entity().entity_uuid;

            {
                let mut selections = crate::selections().write().await;
                let selection = selections.entry(player_uuid).or_default();
                selection.set_pos2(block_pos);
            }

            Ok(())
        })
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_DESC, BlockPosArgumentConsumer).execute(Pos2Executer))
        .execute(Pos2Executer)
}
