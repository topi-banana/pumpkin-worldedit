use std::sync::Arc;

use pumpkin::plugin::{Context, EventPriority, Plugin, PluginFuture, PluginMetadata};

use crate::wand_handler::BlockBreakHandler;
use crate::{storage::WorldEditDataStorage, wand_handler::PlayerInteractHandler};

mod storage;
mod utils;

mod commands;

mod wand_handler;

pub struct Worldedit {
    storage: Arc<WorldEditDataStorage>,
}

impl Plugin for Worldedit {
    fn on_load(&mut self, server: Arc<Context>) -> PluginFuture<'_, Result<(), String>> {
        Box::pin(async move {
            log::info!("Hello, Pumpkin!");

            log::debug!("Registering commands...");
            commands::register_permission(&server).await;
            commands::register_command(&server, &self.storage).await;
            log::debug!("Commands registered!");

            server
                .register_event(Arc::new(BlockBreakHandler), EventPriority::Highest, true)
                .await;
            server
                .register_event(
                    Arc::new(PlayerInteractHandler::new(&self.storage)),
                    EventPriority::Highest,
                    true,
                )
                .await;

            Ok(())
        })
    }

    fn on_unload(&mut self, _server: Arc<Context>) -> PluginFuture<'_, Result<(), String>> {
        Box::pin(async { Ok(()) })
    }
}

#[unsafe(no_mangle)]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Worldedit {
        storage: Arc::new(WorldEditDataStorage::new()),
    })
}

#[unsafe(no_mangle)]
pub static METADATA: PluginMetadata = PluginMetadata {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    authors: env!("CARGO_PKG_AUTHORS"),
    description: env!("CARGO_PKG_DESCRIPTION"),
};
