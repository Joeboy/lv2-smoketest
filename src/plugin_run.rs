use anyhow::{Context, Result, anyhow, bail};
use std::collections::BTreeMap;

const LV2_AUDIO_PORT: &str = "http://lv2plug.in/ns/lv2core#AudioPort";
const LV2_CV_PORT: &str = "http://lv2plug.in/ns/lv2core#CVPort";
const LV2_CONTROL_PORT: &str = "http://lv2plug.in/ns/lv2core#ControlPort";
const LV2_INPUT_PORT: &str = "http://lv2plug.in/ns/lv2core#InputPort";
const LV2_OUTPUT_PORT: &str = "http://lv2plug.in/ns/lv2core#OutputPort";
const LV2_CONNECTION_OPTIONAL: &str = "http://lv2plug.in/ns/lv2core#connectionOptional";

enum PortStorage {
    Audio(Vec<f32>),
    Control(f32),
}

impl PortStorage {
    fn as_mut_ptr(&mut self) -> *mut f32 {
        match self {
            Self::Audio(values) => values.as_mut_ptr(),
            Self::Control(value) => value as *mut f32,
        }
    }
}

pub fn run_plugin_once(plugin_uri: &str, sample_rate: f64, frame_count: usize) -> Result<()> {
    if frame_count == 0 {
        bail!("--frames must be greater than zero");
    }

    let world = lilv::World::new();
    world.load_all();

    let plugin_uri_node = world.new_uri(plugin_uri);
    let plugin = world
        .plugins()
        .plugin(&plugin_uri_node)
        .ok_or_else(|| anyhow!("Plug-in not found for URI: {plugin_uri}"))?;

    let required_features: Vec<String> = plugin
        .required_features()
        .iter()
        .filter_map(|node| node.as_uri().map(str::to_owned))
        .collect();
    if !required_features.is_empty() {
        bail!(
            "Plugin requires unsupported LV2 features for test-plugin-run: {}",
            required_features.join(", ")
        );
    }

    let mut instance = unsafe { plugin.instantiate(sample_rate, []) }
        .ok_or_else(|| anyhow!("Failed to instantiate plugin {plugin_uri}"))
        .context("Plugin instantiate returned null")?;

    let mut port_map: BTreeMap<usize, PortStorage> = BTreeMap::new();

    let audio_port = world.new_uri(LV2_AUDIO_PORT);
    let cv_port = world.new_uri(LV2_CV_PORT);
    let control_port = world.new_uri(LV2_CONTROL_PORT);
    let input_port = world.new_uri(LV2_INPUT_PORT);
    let output_port = world.new_uri(LV2_OUTPUT_PORT);
    let connection_optional = world.new_uri(LV2_CONNECTION_OPTIONAL);

    for port in plugin.iter_ports() {
        let index = port.index();
        let is_input = port.is_a(&input_port);
        let is_output = port.is_a(&output_port);
        let is_audio_like = port.is_a(&audio_port) || port.is_a(&cv_port);
        let is_control = port.is_a(&control_port);

        let storage = if is_audio_like {
            let values = vec![0.0; frame_count];
            let _ = (is_input, is_output);
            PortStorage::Audio(values)
        } else if is_control {
            let value = if is_input {
                port.range()
                    .default
                    .as_ref()
                    .and_then(|node| node.as_float().or_else(|| node.as_int().map(|v| v as f32)))
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            PortStorage::Control(value)
        } else if port.has_property(&connection_optional) {
            continue;
        } else {
            let symbol = port
                .symbol()
                .and_then(|n| n.as_str().map(str::to_owned))
                .unwrap_or_else(|| format!("port-{index}"));
            bail!(
                "Unsupported required port type for test-plugin-run: index={index}, symbol={symbol}"
            );
        };

        port_map.insert(index, storage);

        if let Some(storage) = port_map.get_mut(&index) {
            unsafe {
                instance.connect_port_mut(index, storage.as_mut_ptr());
            }
        }
    }

    let mut active = unsafe { instance.activate() };
    unsafe {
        active.run(frame_count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_frames_is_rejected() {
        let result = run_plugin_once("http://example.invalid/plugin", 44_100.0, 0);
        assert!(result.is_err());
    }
}
