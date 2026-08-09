use crate::config::Config;
use crate::error::{BumperError, BumperResult};
use crate::strategy::Strategy;
use crate::versioner::Version;
use regex::Regex;
use std::fs;

const DEFAULT_PACKAGE_FILE: &str = "pyproject.toml";
const VERSION_FIELD_REGEX: &str = r#"(?m)^version\s*=\s*"([^"]+)""#;
const VERSION_CONSTANT_REGEX: &str = r#"(?m)^((?:__version__|VERSION))\s*=\s*"([^"]+)""#;

pub struct PythonStrategy {
    config: Config,
}

impl PythonStrategy {
    pub fn new(config: Config) -> Self {
        PythonStrategy { config }
    }

    fn package_files(&self) -> Vec<String> {
        if !self.config.package_files.is_empty() {
            self.config.package_files.clone()
        } else {
            vec![DEFAULT_PACKAGE_FILE.to_string()]
        }
    }

    /// Extract the current version from the first matching file, preferring
    /// the PEP 621 `version = "..."` field and falling back to a
    /// `__version__` / `VERSION` constant line.
    fn current_version(&self) -> BumperResult<Version> {
        let version_field = Regex::new(VERSION_FIELD_REGEX).unwrap();
        let version_constant = Regex::new(VERSION_CONSTANT_REGEX).unwrap();

        for file in self.package_files() {
            if !std::path::Path::new(&file).exists() {
                continue;
            }
            let content = fs::read_to_string(&file)?;
            if let Some(captures) = version_field.captures(&content) {
                return Version::parse(captures.get(1).unwrap().as_str());
            }
            if let Some(captures) = version_constant.captures(&content) {
                return Version::parse(captures.get(2).unwrap().as_str());
            }
        }

        Err(BumperError::InvalidVersion(
            "No version field or __version__/VERSION constant found in package files".to_string(),
        ))
    }
}

impl Strategy for PythonStrategy {
    fn get_current_version(&self) -> BumperResult<Version> {
        self.current_version()
    }

    fn update_files(&self, new_version: &Version) -> BumperResult<Vec<String>> {
        let mut updated = Vec::new();
        let version_field = Regex::new(r#"(?m)^version\s*=\s*"[^"]+""#).unwrap();
        let version_constant = Regex::new(r#"(?m)^((?:__version__|VERSION))\s*=\s*"[^"]+""#).unwrap();

        for file in self.package_files() {
            if !std::path::Path::new(&file).exists() {
                continue;
            }
            let content = fs::read_to_string(&file)?;
            // Both patterns are applied per file and are mutually exclusive:
            // the field regex requires a line-start `version` (never matches
            // `__version__` / `VERSION`), and the constant regex is
            // case-sensitive (never matches `version = ...` under `[project]`).
            let new_content = version_field.replace(&content, format!(r#"version = "{}""#, new_version));
            let new_content = version_constant
                .replace(&new_content, format!(r#"$1 = "{}""#, new_version));

            if new_content != content {
                fs::write(&file, new_content.as_ref())?;
                updated.push(file.clone());
            }
        }

        Ok(updated)
    }
}
