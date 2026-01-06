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

use crate::storage::WorldEditDataStorage;

const NAMES: [&str; 1] = ["/undo"];

const DESCRIPTION: &str = "Undoes the last action (from history)";

struct UndoExecutor {
    runtime: tokio::runtime::Runtime,
    storage: Arc<WorldEditDataStorage>,
}

impl UndoExecutor {
    fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            runtime: tokio::runtime::Runtime::new().unwrap(),
            storage: storage.clone(),
        }
    }
}

impl CommandExecutor for UndoExecutor {
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

            let Some(diff) = self.storage.histories.undo(player_uuid).await else {
                return Err(CommandError::CommandFailed(
                    TextComponent::text("Unable to find session for ").add_child(player.get_name()),
                ));
            };

            self.runtime
                .spawn(async move {
                    for diff in diff.iter() {
                        player
                            .world()
                            .set_block_state(&diff.position, diff.before, BlockFlags::empty())
                            .await;
                    }
                })
                .await
                .unwrap();

            Ok(())
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(UndoExecutor::new(storage))
}
