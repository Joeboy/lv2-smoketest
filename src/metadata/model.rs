use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct PluginMetadata {
    pub uri: String,
    pub name: Option<String>,
    pub classes: Vec<PluginClass>,
    pub license: Vec<String>,
    pub author: AuthorMetadata,
    pub bundle_uri: Option<String>,
    pub bundle_path: Option<String>,
    pub library_uri: Option<String>,
    pub ports: Vec<PortMetadata>,
    pub raw_extras: RawExtras,
}

#[derive(Debug, Serialize)]
pub struct PluginSummary {
    pub uri: String,
    pub name: Option<String>,
    pub classes: Vec<PluginClass>,
    pub bundle_uri: Option<String>,
    pub bundle_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PluginClass {
    pub uri: Option<String>,
    pub label: Option<String>,
    pub parent_uri: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct AuthorMetadata {
    pub name: Option<String>,
    pub email: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PortMetadata {
    pub index: usize,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub direction: PortDirection,
    pub port_type: PortType,
    pub classes: Vec<String>,
    pub properties: Vec<String>,
    pub range: PortRange,
    pub raw_extras: RawExtras,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortDirection {
    Input,
    Output,
    Unknown,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortType {
    Audio,
    Control,
    Cv,
    Atom,
    Event,
    Unknown,
}

#[derive(Debug, Serialize, Default)]
pub struct PortRange {
    pub default: Option<f32>,
    pub minimum: Option<f32>,
    pub maximum: Option<f32>,
}

pub type RawExtras = BTreeMap<String, Vec<String>>;
