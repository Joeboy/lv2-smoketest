pub mod cli;
pub mod lv2;
pub mod metadata;
pub mod plugin_run;

use anyhow::Result;

pub fn show_plugin_metadata_json(plugin_uri: &str) -> Result<String> {
    let world = lv2::world::Lv2World::load();
    let plugin = world
        .find_plugin(plugin_uri)
        .ok_or_else(|| anyhow::anyhow!("Plugin not found for URI: {plugin_uri}"))?;

    let metadata = metadata::extract::extract_plugin_metadata(world.world(), &plugin)?;
    let json = serde_json::to_string_pretty(&metadata)?;
    Ok(json)
}

pub fn list_installed_plugins_json() -> Result<String> {
    let world = lv2::world::Lv2World::load();
    let plugins = metadata::extract::extract_plugin_summaries(world.world());
    let json = serde_json::to_string_pretty(&plugins)?;
    Ok(json)
}

pub fn test_plugin_run(plugin_uri: &str, sample_rate: f64, frames: usize) -> Result<()> {
    plugin_run::run_plugin_once(plugin_uri, sample_rate, frames)
}
