use crate::config::Config;
use crate::error::{BumperError, BumperResult};
use crate::strategy::Strategy;
use crate::versioner::Version;
use serde_json::Value;
use std::fs;

pub struct NodeStrategy {
    config: Config,
}

impl NodeStrategy {
    pub fn new(config: Config) -> Self {
        NodeStrategy { config }
    }
}

impl Strategy for NodeStrategy {
    fn get_current_version(&self) -> BumperResult<Version> {
        let main_file = &self.config.package_files[0];

        if !std::path::Path::new(main_file).exists() {
            return Err(BumperError::FileNotFound(main_file.clone()));
        }

        let content = fs::read_to_string(main_file)?;
        let package: Value = serde_json::from_str(&content)?;

        let version_str = package["version"]
            .as_str()
            .ok_or_else(|| BumperError::InvalidVersion("No version field found".to_string()))?;

        Version::parse(version_str)
    }

    fn update_files(&self, new_version: &Version) -> BumperResult<Vec<String>> {
        let mut updated = Vec::new();

        for file in &self.config.package_files {
            if std::path::Path::new(file).exists() {
                let content = fs::read_to_string(file)?;
                let mut package: Value = serde_json::from_str(&content)?;

                package["version"] = Value::String(new_version.to_string());

                // package-lock.json repeats the root package's version under packages[""]
                if let Some(root_version) = package
                    .get_mut("packages")
                    .and_then(|packages| packages.get_mut(""))
                    .and_then(|root| root.get_mut("version"))
                {
                    *root_version = Value::String(new_version.to_string());
                }

                let updated_content = serde_json::to_string_pretty(&package)?;
                fs::write(file, format!("{}\n", updated_content))?;

                updated.push(file.clone());
            }
        }

        Ok(updated)
    }
}
