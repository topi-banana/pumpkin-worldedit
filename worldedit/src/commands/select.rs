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

use crate::storage::WorldEditDataStorage;

const NAMES: [&str; 4] = ["/sel", ";", "/desel", "/deselect"];

const DESCRIPTION: &str = "Choose a region selector";

struct SelectExecuter {
    storage: Arc<WorldEditDataStorage>,
}

impl SelectExecuter {
    fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl CommandExecutor for SelectExecuter {
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
            let message = "Selection cleared.".to_string();
            sender.send_message(TextComponent::text(message)).await;

            let player_uuid = player.get_entity().entity_uuid;
            self.storage.sections.remove_section(&player_uuid).await;

            Ok(())
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(SelectExecuter::new(storage))
}
