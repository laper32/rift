use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TomlFolder {
    pub name: String,
    pub members: Vec<String>,
    pub exclude: Vec<String>,
    #[serde(skip_serializing)]
    #[serde(flatten)]
    pub others: toml::Value,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TomlWorkspace {
    pub name: Option<String>,
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub plugins: Option<String>,
    pub configure: Option<String>,
    pub dependencies: Option<String>,
    #[serde(skip_serializing)]
    #[serde(flatten)]
    pub others: toml::Value,
}

/*

internal sealed class TomlWorkspace
{
    [JsonPropertyName("name")]
    public string? Name { get; set; }

    [JsonPropertyName("members")]
    public List<string>? Members { get; set; }

    [JsonPropertyName("exclude")]
    public List<string>? Exclude { get; set; }

    [JsonPropertyName("plugins")]
    public string? Plugins { get; set; }

    [JsonPropertyName("configure")]
    public string? Configure { get; set; }

    [JsonPropertyName("dependencies")]
    public string? Dependencies { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement> Others { get; set; } = [];
}
*/
