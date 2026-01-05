use pumpkin::command::CommandExecutor;
use pumpkin::command::CommandResult;
use pumpkin::command::CommandSender;
use pumpkin::command::args::ConsumedArgs;
use pumpkin::command::dispatcher::CommandError;
use pumpkin::command::tree::CommandTree;
use pumpkin::entity::EntityBase;
use pumpkin::server::Server;
use pumpkin_util::text::TextComponent;

const NAMES: [&str; 4] = ["/sel", ";", "/desel", "/deselect"];

const DESCRIPTION: &str = "Choose a region selector";

struct SelectExecuter;

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
            {
                let mut selections = crate::selections().write().await;
                selections.remove(&player_uuid);
            }

            Ok(())
        })
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(SelectExecuter)
}
