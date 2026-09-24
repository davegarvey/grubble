use crate::config::Config;
use crate::error::{BumperError, BumperResult};
use crate::strategy::Strategy;
use crate::versioner::Version;
use regex::Regex;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
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

                fs::write(file, format_like(&content, &package)?)?;

                updated.push(file.clone());
            }
        }

        Ok(updated)
    }
}

/// Serialise `value` in the layout of `original`, following npm: the line ending
/// and indentation come from the first line after the opening brace, and a file
/// with no line break there is written compactly. Text outside the outermost
/// braces, such as the trailing newline, is kept as it was.
fn format_like(original: &str, value: &Value) -> BumperResult<String> {
    let layout = Regex::new(r"^\s*\{(\r?\n)([ \t]*)").unwrap();

    let body = match layout.captures(original) {
        Some(captures) => {
            let mut buf = Vec::new();
            let formatter = PrettyFormatter::with_indent(captures[2].as_bytes());
            value.serialize(&mut serde_json::Serializer::with_formatter(
                &mut buf, formatter,
            ))?;
            // Raw newlines cannot occur inside JSON strings, so this only touches layout
            String::from_utf8(buf)
                .expect("serde_json emits valid UTF-8")
                .replace('\n', &captures[1])
        }
        None => serde_json::to_string(value)?,
    };

    let prefix = original.find('{').map_or("", |start| &original[..start]);
    let suffix = original.rfind('}').map_or("\n", |end| &original[end + 1..]);

    Ok(format!("{}{}{}", prefix, body, suffix))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bumps `original`'s version to 1.1.0 and returns the rewritten text.
    fn bump(original: &str) -> String {
        let mut value: Value = serde_json::from_str(original).unwrap();
        value["version"] = Value::String("1.1.0".to_string());
        format_like(original, &value).unwrap()
    }

    #[test]
    fn keeps_two_space_indent() {
        let original = "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\",\n  \"files\": [\n    \"dist\"\n  ]\n}\n";
        assert_eq!(bump(original), original.replace("1.0.0", "1.1.0"));
    }

    #[test]
    fn keeps_four_space_indent() {
        let original = "{\n    \"name\": \"demo\",\n    \"version\": \"1.0.0\"\n}\n";
        assert_eq!(bump(original), original.replace("1.0.0", "1.1.0"));
    }

    #[test]
    fn keeps_tab_indent_and_crlf() {
        let original = "{\r\n\t\"name\": \"demo\",\r\n\t\"version\": \"1.0.0\",\r\n\t\"scripts\": {\r\n\t\t\"test\": \"jest\"\r\n\t}\r\n}\r\n";
        assert_eq!(bump(original), original.replace("1.0.0", "1.1.0"));
    }

    #[test]
    fn keeps_missing_trailing_newline() {
        let original = "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\"\n}";
        assert_eq!(bump(original), original.replace("1.0.0", "1.1.0"));
    }

    #[test]
    fn keeps_single_line_file_compact() {
        let original = "{\"name\":\"demo\",\"version\":\"1.0.0\"}\n";
        assert_eq!(bump(original), original.replace("1.0.0", "1.1.0"));
    }
}
