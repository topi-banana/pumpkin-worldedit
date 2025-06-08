use async_trait::async_trait;
use pumpkin::command::CommandExecutor;
use pumpkin::command::CommandSender;
use pumpkin::command::args::Arg;
use pumpkin::command::args::ConsumedArgs;
use pumpkin::command::args::position_block::BlockPosArgumentConsumer;
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::tree::CommandTree;
use pumpkin::command::tree::builder::argument;
use pumpkin::entity::EntityBase;
use pumpkin::server::Server;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::text::TextComponent;

const NAMES: [&str; 1] = ["/pos1"];

const DESCRIPTION: &str = "Set position 1";

const ARG_DESC: &str = "Coordinates to set position 1 to";

struct Pos1Executer;

#[async_trait]
impl CommandExecutor for Pos1Executer {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
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

        {
            let mut selections = crate::selections().write().await;
            let selection = selections.entry(player_uuid).or_default();
            selection.set_pos1(block_pos);
        }

        Ok(())
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_DESC, BlockPosArgumentConsumer).execute(Pos1Executer))
        .execute(Pos1Executer)
}
