use async_trait::async_trait;
use pumpkin::command::args::block::BlockArgumentConsumer;
use pumpkin::command::args::ConsumedArgs;
use pumpkin::command::args::FindArg;
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::tree::builder::argument;
use pumpkin::command::tree::CommandTree;
use pumpkin::command::CommandExecutor;
use pumpkin::command::CommandSender;
use pumpkin::entity::EntityBase;
use pumpkin::server::Server;
use pumpkin_world::world::BlockFlags;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_util::text::TextComponent;

use crate::normalization_selection;

const NAMES: [&str; 3] = ["/replace", "/re", "/rep"];

const DESCRIPTION: &str = "Replace all blocks in the selection with another";

const ARG_DESC_FROM: &str = "The mask representing blocks to replace";
const ARG_DESC_TO: &str = "The pattern of blocks to set";

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

        let block_from = BlockArgumentConsumer::find_arg(args, ARG_DESC_FROM)?;
        let block_to = BlockArgumentConsumer::find_arg(args, ARG_DESC_TO)?;

        let (mut pos1, mut pos2) = crate::fetch_selections(&player.get_entity().entity_uuid).await?;

        normalization_selection(&mut pos1.0, &mut pos2.0);

        let world = player.world().await;

        let (x1, x2) = (pos1.0.x, pos2.0.x);
        let (z1, z2) = (pos1.0.z, pos2.0.z);
        let (y1, y2) = (pos1.0.y, pos2.0.y);

        let mut cnt = 0;

        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    let block_position = BlockPos(Vector3 { x, y, z });
                    if world.get_block_state(&block_position).await.id != block_from.id {
                        world.set_block_state(
                            &block_position,
                            block_to.id,
                            BlockFlags::FORCE_STATE
                        ).await;
                        cnt += 1;
                    }
                }
            }
        }

        sender.send_message(TextComponent::text(format!("{} blocks have been changed.", cnt))).await;

        Ok(())
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(
            argument(ARG_DESC_FROM, BlockArgumentConsumer)
            .then(
                argument(ARG_DESC_TO, BlockArgumentConsumer)
                    .execute(SetExecuter)
            )
        )
}

