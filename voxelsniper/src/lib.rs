use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};

#[plugin_method]
async fn on_load(&mut self, context: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    // log::debug!("Hello World");

    Ok(())
}

#[plugin_impl]
pub struct Worldedit {}

impl Worldedit {
    pub fn new() -> Self {
        Worldedit {}
    }
}

impl Default for Worldedit {
    fn default() -> Self {
        Self::new()
    }
}
