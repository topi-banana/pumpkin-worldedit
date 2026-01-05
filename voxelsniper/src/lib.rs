use std::sync::Arc;

use pumpkin::plugin::{Context, Plugin, PluginFuture, PluginMetadata};

pub struct Voxelsniper;

impl Plugin for Voxelsniper {
    fn on_load(&mut self, _server: Arc<Context>) -> PluginFuture<'_, Result<(), String>> {
        Box::pin(async move {
            log::info!("Hello, Pumpkin!");

            Ok(())
        })
    }

    fn on_unload(&mut self, _server: Arc<Context>) -> PluginFuture<'_, Result<(), String>> {
        Box::pin(async { Ok(()) })
    }
}

#[unsafe(no_mangle)]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Voxelsniper)
}

#[unsafe(no_mangle)]
pub static METADATA: PluginMetadata = PluginMetadata {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    authors: env!("CARGO_PKG_AUTHORS"),
    description: env!("CARGO_PKG_DESCRIPTION"),
};
