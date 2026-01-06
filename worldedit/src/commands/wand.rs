use std::sync::Arc;

use pumpkin::{
    command::{
        CommandExecutor, CommandResult, CommandSender, args::ConsumedArgs,
        dispatcher::CommandError, tree::CommandTree,
    },
    server::Server,
};
use pumpkin_data::item::Item;
use pumpkin_util::text::TextComponent;
use pumpkin_world::item::ItemStack;

use crate::storage::WorldEditDataStorage;

const NAMES: [&str; 1] = ["/wand"];

const DESCRIPTION: &str = "Get the wand item";

struct WandExecutor {
    // storage: Arc<WorldEditDataStorage>,
}

impl WandExecutor {
    fn new(_storage: &Arc<WorldEditDataStorage>) -> Self {
        Self {
            // storage: storage.clone(),
        }
    }
}

impl CommandExecutor for WandExecutor {
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

            let mut itemstack = ItemStack::new(1, &Item::WOODEN_AXE);
            player
                .inventory()
                .insert_stack_anywhere(&mut itemstack)
                .await;

            let message =
                TextComponent::text("Left click: select pos #1; Right click: select pos #2");
            sender.send_message(message).await;
            Ok(())
        })
    }
}

pub fn init_command_tree(storage: &Arc<WorldEditDataStorage>) -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(WandExecutor::new(storage))
}
