use std::hash::Hash;
use std::str;

use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::script::{Alias, Timer, Trigger};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
#[serde(from = "PluginFile")]
pub struct PluginPack {
    pub metadata: PluginMetadata,
    pub triggers: Vec<Trigger>,
    pub aliases: Vec<Alias>,
    pub timers: Vec<Timer>,
    pub script: String,
}

impl From<PluginFile> for PluginPack {
    fn from(value: PluginFile) -> Self {
        Self {
            metadata: value.plugin,
            triggers: value
                .triggers
                .into_iter()
                .flat_map(|x| x.children)
                .collect(),
            timers: value.timers.into_iter().flat_map(|x| x.children).collect(),
            aliases: value.aliases.into_iter().flat_map(|x| x.children).collect(),
            script: value.script.join("\n"),
        }
    }
}

/// Corresponds to a plugin .xml file.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct PluginFile {
    plugin: PluginMetadata,
    #[serde(default)]
    triggers: Vec<Triggers>,
    #[serde(default)]
    aliases: Vec<Aliases>,
    #[serde(default)]
    timers: Vec<Timers>,
    #[serde(default)]
    script: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
struct Triggers {
    muclient_version: Option<String>,
    world_file_version: Option<u32>,
    date_saved: Option<NaiveDateTime>,
    #[serde(rename = "trigger")]
    children: Vec<Trigger>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
struct Aliases {
    muclient_version: Option<String>,
    world_file_version: Option<u32>,
    date_saved: Option<NaiveDateTime>,
    #[serde(rename = "alias")]
    children: Vec<Alias>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
struct Timers {
    muclient_version: Option<String>,
    world_file_version: Option<u32>,
    date_saved: Option<NaiveDateTime>,
    #[serde(rename = "timer")]
    children: Vec<Timer>,
}

fn today() -> NaiveDate {
    Utc::today().naive_utc()
}

/// World plugins.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PluginMetadata {
    /// Plugin name.
    pub name: String,
    /// Who wrote it?
    pub author: String,
    /// Short description of the plugin's functionality.
    #[serde(default)]
    pub purpose: String,
    /// Long description of the plugin's functionality.
    #[serde(default)]
    pub description: String,
    /// Unique ID.
    pub id: String,
    /// Date written.
    #[serde(default = "today")]
    pub written: NaiveDate,
    /// Date last modified.
    #[serde(default = "today")]
    pub modified: NaiveDate,
    /// Plugin version.
    pub version: String,
    /// Minimum qMUSHclient version required.
    pub requires: String,
    /// Evaluation order. Lower is sooner.
    /// Negative sequences are evaluated before the main world triggers/aliases.
    #[serde(default)]
    pub sequence: i16,
    #[serde(skip)]
    pub is_world_plugin: bool,
}

const EMPTY_METADATA: PluginMetadata = PluginMetadata {
    name: String::new(),
    author: String::new(),
    purpose: String::new(),
    description: String::new(),
    id: String::new(),
    written: chrono::naive::MIN_DATE,
    modified: chrono::naive::MIN_DATE,
    version: String::new(),
    requires: String::new(),
    sequence: 0,
    is_world_plugin: false,
};

impl Default for PluginMetadata {
    fn default() -> Self {
        EMPTY_METADATA
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = include_str!("../../tests/DemoPlugin.xml");
        quick_xml::de::from_str::<PluginPack>(src).expect("failed to parse file");
    }
}
