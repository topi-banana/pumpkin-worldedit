use std::sync::Arc;

use pumpkin::{
    command::{
        CommandExecutor, CommandResult, CommandSender, args::ConsumedArgs,
        dispatcher::CommandError, tree::CommandTree,
    },
    entity::EntityBase,
    server::Server,
};
use pumpkin_util::text::TextComponent;
use pumpkin_world::world::BlockFlags;

use crate::storage::{Diff, WorldEditDataStorage};

const NAMES: [&str; 1] = ["/paste"];

const DESCRIPTION: &str = "Paste the clipboard's contents";

struct PasteExecutor {
    storage: Arc<WorldEditDataStorage>,
}

impl PasteExecutor {
    fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl CommandExecutor for PasteExecutor {
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

            let player_uuid = player.get_entity().entity_uuid;

            let world = player.world();

            let Some(clipboard) = self.storage.clipboard.get(player_uuid).await else {
                return Ok(());
            };

            let mut block_diff = Vec::new();

            for &(pos, block_state_id) in clipboard.get_data() {
                let old_block_state_id = world.get_block_state(&pos).await.id;
                if old_block_state_id != block_state_id {
                    world
                        .set_block_state(&pos, block_state_id, BlockFlags::empty())
                        .await;
                    block_diff.push(Diff {
                        position: pos,
                        before: old_block_state_id,
                        after: block_state_id,
                    });
                }
            }

            self.storage
                .histories
                .push(player_uuid, Arc::from(block_diff.into_boxed_slice()))
                .await;

            let message =
                TextComponent::text("Left click: select pos #1; Right click: select pos #2");
            sender.send_message(message).await;
            Ok(())
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(PasteExecutor::new(storage))
}
