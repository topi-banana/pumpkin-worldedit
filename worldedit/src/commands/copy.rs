use std::sync::Arc;

use pumpkin::{
    command::{
        CommandExecutor, CommandResult, CommandSender, args::ConsumedArgs,
        dispatcher::CommandError, tree::CommandTree,
    },
    server::Server,
};
use pumpkin_util::text::TextComponent;

use crate::storage::WorldEditDataStorage;

const NAMES: [&str; 1] = ["/copy"];

const DESCRIPTION: &str = "Copy the selection to the clipboard";

struct CopyExecutor {
    storage: Arc<WorldEditDataStorage>,
}

impl CopyExecutor {
    fn new(storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            storage: storage.clone(),
        }
    }
}

impl CommandExecutor for CopyExecutor {
    fn execute<'a>(
        &'a self,
        _sender: &'a CommandSender,
        _server: &'a Server,
        _args: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            drop(self.storage.clone());
            // TODO: Implement copy command
            Err(CommandError::CommandFailed(TextComponent::text(
                "not yet implemented",
            )))
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(CopyExecutor::new(storage))
}
