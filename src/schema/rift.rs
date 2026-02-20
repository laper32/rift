/*
internal sealed class TomlPlugin
{
    [JsonPropertyName("name")]
    public required string Name { get; set; }

    [JsonPropertyName("version")]
    public required string Version { get; set; }

    [JsonPropertyName("authors")]
    public required List<string> Authors { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }

    [JsonPropertyName("configure")]
    public string? Configure { get; set; }

    [JsonPropertyName("dependencies")]
    public string? Dependencies { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement> Others { get; set; } = [];
}
*/

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TomlPlugin {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    #[serde(skip_serializing)]
    #[serde(flatten)]
    pub others: HashMap<String, toml::Value>,
}
