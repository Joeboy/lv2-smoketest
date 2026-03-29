use crate::metadata::model::{
    AuthorMetadata, PluginClass, PluginMetadata, PluginSummary, PortDirection, PortMetadata,
    PortRange, PortType, RawExtras,
};
use anyhow::Result;
use lilv::node::{Node, Nodes};
use lilv::plugin::Plugin;
use lilv::port::Port;

const DOAP_LICENSE: &str = "http://usefulinc.com/ns/doap#license";
const LV2_INPUT_PORT: &str = "http://lv2plug.in/ns/lv2core#InputPort";
const LV2_OUTPUT_PORT: &str = "http://lv2plug.in/ns/lv2core#OutputPort";
const LV2_AUDIO_PORT: &str = "http://lv2plug.in/ns/lv2core#AudioPort";
const LV2_CONTROL_PORT: &str = "http://lv2plug.in/ns/lv2core#ControlPort";
const LV2_CV_PORT: &str = "http://lv2plug.in/ns/lv2core#CVPort";
const ATOM_ATOM_PORT: &str = "http://lv2plug.in/ns/ext/atom#AtomPort";
const EV_EVENT_PORT: &str = "http://lv2plug.in/ns/ext/event#EventPort";

pub fn extract_plugin_metadata(world: &lilv::World, plugin: &Plugin) -> Result<PluginMetadata> {
    let uri = node_to_string(&plugin.uri()).unwrap_or_default();
    let name = node_to_string(&plugin.name());

    let classes = plugin_classes(plugin);

    let license_predicate = world.new_uri(DOAP_LICENSE);
    let license = nodes_to_strings(plugin.value(&license_predicate));

    let author = AuthorMetadata {
        name: plugin.author_name().and_then(|node| node_to_string(&node)),
        email: plugin.author_email().and_then(|node| node_to_string(&node)),
        homepage: plugin
            .author_homepage()
            .and_then(|node| node_to_string(&node)),
    };

    let bundle_node = plugin.bundle_uri();
    let bundle_uri = node_to_string(&bundle_node);
    let bundle_path = bundle_node.path().map(|(_, path)| path);
    let library_uri = plugin.library_uri().and_then(|node| node_to_string(&node));

    let ports = plugin
        .iter_ports()
        .map(|port| extract_port_metadata(world, &port))
        .collect();

    let raw_extras = build_plugin_raw_extras(plugin);

    Ok(PluginMetadata {
        uri,
        name,
        classes,
        license,
        author,
        bundle_uri,
        bundle_path,
        library_uri,
        ports,
        raw_extras,
    })
}

pub fn extract_plugin_summaries(world: &lilv::World) -> Vec<PluginSummary> {
    let mut plugins: Vec<_> = world
        .plugins()
        .iter()
        .map(|plugin| {
            let bundle_node = plugin.bundle_uri();
            PluginSummary {
                uri: node_to_string(&plugin.uri()).unwrap_or_default(),
                name: node_to_string(&plugin.name()),
                classes: plugin_classes(&plugin),
                bundle_uri: node_to_string(&bundle_node),
                bundle_path: bundle_node.path().map(|(_, path)| path),
            }
        })
        .collect();

    plugins.sort_by(|left, right| left.uri.cmp(&right.uri));
    plugins
}

fn extract_port_metadata(world: &lilv::World, port: &Port) -> PortMetadata {
    let input_port = world.new_uri(LV2_INPUT_PORT);
    let output_port = world.new_uri(LV2_OUTPUT_PORT);
    let audio_port = world.new_uri(LV2_AUDIO_PORT);
    let control_port = world.new_uri(LV2_CONTROL_PORT);
    let cv_port = world.new_uri(LV2_CV_PORT);
    let atom_port = world.new_uri(ATOM_ATOM_PORT);
    let event_port = world.new_uri(EV_EVENT_PORT);

    let direction = if port.is_a(&input_port) {
        PortDirection::Input
    } else if port.is_a(&output_port) {
        PortDirection::Output
    } else {
        PortDirection::Unknown
    };

    let port_type = if port.is_a(&audio_port) {
        PortType::Audio
    } else if port.is_a(&control_port) {
        PortType::Control
    } else if port.is_a(&cv_port) {
        PortType::Cv
    } else if port.is_a(&atom_port) {
        PortType::Atom
    } else if port.is_a(&event_port) {
        PortType::Event
    } else {
        PortType::Unknown
    };

    let classes = nodes_to_strings(port.classes());
    let properties = nodes_to_strings(port.properties());
    let raw_extras = build_port_raw_extras(port, &classes, &properties);

    let range_raw = port.range();
    let range = PortRange {
        default: range_raw.default.as_ref().and_then(node_to_f32),
        minimum: range_raw.minimum.as_ref().and_then(node_to_f32),
        maximum: range_raw.maximum.as_ref().and_then(node_to_f32),
    };

    PortMetadata {
        index: port.index(),
        symbol: port.symbol().and_then(|node| node_to_string(&node)),
        name: port.name().and_then(|node| node_to_string(&node)),
        direction,
        port_type,
        classes,
        properties,
        range,
        raw_extras,
    }
}

fn build_plugin_raw_extras(plugin: &Plugin) -> RawExtras {
    let mut extras = RawExtras::new();

    extras.insert("data_uris".to_owned(), nodes_to_strings(plugin.data_uris()));
    extras.insert(
        "supported_features".to_owned(),
        nodes_to_strings(plugin.supported_features()),
    );
    extras.insert(
        "required_features".to_owned(),
        nodes_to_strings(plugin.required_features()),
    );
    extras.insert(
        "optional_features".to_owned(),
        nodes_to_strings(plugin.optional_features()),
    );

    if let Some(extension_data) = plugin.extension_data() {
        extras.insert(
            "extension_data".to_owned(),
            nodes_to_strings(extension_data),
        );
    }

    if let Some(project) = plugin.project().and_then(|node| node_to_string(&node)) {
        extras.insert("project".to_owned(), vec![project]);
    }

    extras.insert(
        "is_replaced".to_owned(),
        vec![plugin.is_replaced().to_string()],
    );

    extras.retain(|_, values| !values.is_empty());
    extras
}

fn plugin_classes(plugin: &Plugin) -> Vec<PluginClass> {
    let class = plugin.class();
    vec![PluginClass {
        uri: class.uri().and_then(|node| node_to_string(&node)),
        label: node_to_string(&class.label()),
        parent_uri: class.parent_uri().and_then(|node| node_to_string(&node)),
    }]
}

fn build_port_raw_extras(port: &Port, classes: &[String], properties: &[String]) -> RawExtras {
    let mut extras = RawExtras::new();

    if !classes.is_empty() {
        extras.insert("classes".to_owned(), classes.to_vec());
    }

    if !properties.is_empty() {
        extras.insert("properties".to_owned(), properties.to_vec());
    }

    // Include full Turtle token for port node so callers can cross-reference RDF details.
    extras.insert("port_node".to_owned(), vec![port.node().turtle_token()]);

    extras
}

fn nodes_to_strings(nodes: Nodes) -> Vec<String> {
    nodes
        .iter()
        .filter_map(|node| node_to_string(&node))
        .collect()
}

fn node_to_string(node: &Node) -> Option<String> {
    if let Some(uri) = node.as_uri() {
        return Some(uri.to_owned());
    }

    if let Some(text) = node.as_str() {
        return Some(text.to_owned());
    }

    if let Some(int_value) = node.as_int() {
        return Some(int_value.to_string());
    }

    if let Some(float_value) = node.as_float() {
        return Some(float_value.to_string());
    }

    if let Some(bool_value) = node.as_bool() {
        return Some(bool_value.to_string());
    }

    Some(node.turtle_token())
}

fn node_to_f32(node: &Node) -> Option<f32> {
    node.as_float()
        .or_else(|| node.as_int().map(|value| value as f32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_from_int_node_is_supported() {
        let world = lilv::World::new();
        let node = world.new_int(7);
        assert_eq!(node_to_f32(&node), Some(7.0));
    }
}
