use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};

#[plugin_method]
async fn on_load(&mut self, context: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    // log::debug!("Hello World");

    Ok(())
}

#[plugin_impl]
pub struct Voxelsniper {}

impl Voxelsniper {
    pub fn new() -> Self {
        Voxelsniper {}
    }
}

impl Default for Voxelsniper {
    fn default() -> Self {
        Self::new()
    }
}
