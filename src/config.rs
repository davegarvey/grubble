use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default = "default_package_files")]
    pub package_files: Vec<String>,

    #[serde(default = "default_commit_prefix")]
    pub commit_prefix: String,

    #[serde(default = "default_tag_prefix")]
    pub tag_prefix: String,

    #[serde(default)]
    pub push: bool,

    #[serde(default)]
    pub tag: bool,

    #[serde(default = "default_preset")]
    pub preset: String,

    #[serde(default)]
    pub release_notes: bool,

    #[serde(default = "default_types")]
    pub types: HashMap<String, String>,

    #[serde(skip)]
    pub raw: bool,
}

fn default_package_files() -> Vec<String> {
    vec![]
}

fn default_commit_prefix() -> String {
    "chore: bump version".to_string()
}

fn default_tag_prefix() -> String {
    "v".to_string()
}

fn default_preset() -> String {
    "git".to_string()
}

fn default_types() -> HashMap<String, String> {
    let mut types = HashMap::new();
    types.insert("feat".to_string(), "minor".to_string());
    types.insert("fix".to_string(), "patch".to_string());
    types.insert("build".to_string(), "none".to_string());
    types.insert("chore".to_string(), "none".to_string());
    types.insert("ci".to_string(), "none".to_string());
    types.insert("docs".to_string(), "none".to_string());
    types.insert("style".to_string(), "none".to_string());
    types.insert("refactor".to_string(), "none".to_string());
    types.insert("perf".to_string(), "none".to_string());
    types.insert("test".to_string(), "none".to_string());
    types
}

impl Default for Config {
    fn default() -> Self {
        Config {
            package_files: default_package_files(),
            commit_prefix: default_commit_prefix(),
            tag_prefix: default_tag_prefix(),
            push: false,
            tag: false,
            preset: default_preset(),
            release_notes: false,
            types: default_types(),
            raw: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self::load_from_path(".versionrc.json")
    }

    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(user_config) = serde_json::from_str::<Config>(&content) {
                return user_config;
            } else {
                eprintln!("Warning: Invalid .versionrc.json file, using default config");
            }
        }

        Config::default()
    }
}
