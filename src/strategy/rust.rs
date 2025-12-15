use crate::config::Config;
use crate::error::{BumperError, BumperResult};
use crate::strategy::Strategy;
use crate::versioner::Version;
use regex::Regex;
use std::fs;

pub struct RustStrategy {
    config: Config,
}

impl RustStrategy {
    pub fn new(config: Config) -> Self {
        RustStrategy { config }
    }

    fn get_cargo_file(&self) -> &str {
        // Use first package file if specified, otherwise default to Cargo.toml
        if !self.config.package_files.is_empty() {
            &self.config.package_files[0]
        } else {
            "Cargo.toml"
        }
    }
}

impl Strategy for RustStrategy {
    fn get_current_version(&self) -> BumperResult<Version> {
        let cargo_file = self.get_cargo_file();

        if !std::path::Path::new(cargo_file).exists() {
            return Err(BumperError::FileNotFound(cargo_file.to_string()));
        }

        let content = fs::read_to_string(cargo_file)?;
        let version_regex = Regex::new(r#"(?m)^version\s*=\s*"([^"]+)""#).unwrap();

        if let Some(captures) = version_regex.captures(&content) {
            let version_str = captures.get(1).unwrap().as_str();
            Version::parse(version_str)
        } else {
            Err(BumperError::InvalidVersion(
                "No version field found in Cargo.toml".to_string(),
            ))
        }
    }

    fn update_files(&self, new_version: &Version) -> BumperResult<Vec<String>> {
        let mut updated = Vec::new();
        let version_regex = Regex::new(r#"(?m)^version\s*=\s*"[^"]+""#).unwrap();

        for file in &self.config.package_files {
            if std::path::Path::new(file).exists() {
                let content = fs::read_to_string(file)?;

                let new_content =
                    version_regex.replace(&content, format!(r#"version = "{}""#, new_version));

                fs::write(file, new_content.as_ref())?;
                updated.push(file.clone());
            }
        }

        // Update Cargo.lock if it exists (for binary crates)
        if std::path::Path::new("Cargo.lock").exists() {
            // Run cargo update to refresh Cargo.lock with new version
            std::process::Command::new("cargo")
                .args(["update", "--workspace"])
                .output()
                .ok(); // Ignore errors, best effort
            updated.push("Cargo.lock".to_string());
        }

        Ok(updated)
    }
}
