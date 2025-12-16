use crate::analyser::BumpType;
use crate::error::BumperResult;
use crate::versioner::Version;
use chrono::Local;
use regex::Regex;
use std::fs;
use std::path::Path;

const CHANGELOG_FILE: &str = "CHANGELOG.md";

/// Represents a parsed changelog entry
#[derive(Debug)]
#[allow(dead_code)]
struct ChangelogEntry {
    version: String,
    date: String,
    changes: Vec<Change>,
}

#[derive(Debug)]
struct Change {
    category: ChangeCategory,
    description: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
enum ChangeCategory {
    Added,
    Changed,
    Deprecated,
    Removed,
    Fixed,
    Security,
}

impl ChangeCategory {
    fn as_str(&self) -> &'static str {
        match self {
            ChangeCategory::Added => "### Added",
            ChangeCategory::Changed => "### Changed",
            ChangeCategory::Deprecated => "### Deprecated",
            ChangeCategory::Removed => "### Removed",
            ChangeCategory::Fixed => "### Fixed",
            ChangeCategory::Security => "### Security",
        }
    }

    fn from_commit_type(commit_type: &str) -> Self {
        match commit_type {
            "feat" => ChangeCategory::Added,
            "fix" => ChangeCategory::Fixed,
            "perf" => ChangeCategory::Changed,
            "refactor" => ChangeCategory::Changed,
            "revert" => ChangeCategory::Removed,
            "security" => ChangeCategory::Security,
            _ => ChangeCategory::Changed,
        }
    }
}

/// Categorize commits and generate changelog entry
pub fn generate_changelog_entry(
    version: &Version,
    commits: &[String],
    _bump_type: BumpType,
) -> BumperResult<()> {
    generate_changelog_entry_at_path(version, commits, Path::new(CHANGELOG_FILE))
}

/// Internal function that accepts a custom path for testing
fn generate_changelog_entry_at_path(
    version: &Version,
    commits: &[String],
    changelog_path: &Path,
) -> BumperResult<()> {
    let date = Local::now().format("%Y-%m-%d").to_string();

    // Parse commits into categorized changes
    let mut changes: Vec<Change> = Vec::new();
    let commit_regex = Regex::new(r"^([a-z]+)(?:\([^)]+\))?(!?): (.+)$").unwrap();

    for commit in commits {
        if commit.starts_with("chore: bump version") || commit.starts_with("chore: sync package") {
            continue;
        }

        if let Some(captures) = commit_regex.captures(commit) {
            let commit_type = captures.get(1).map(|m| m.as_str()).unwrap_or("");
            let has_breaking = captures.get(2).map(|m| m.as_str()).unwrap_or("") == "!";
            let description = captures.get(3).map(|m| m.as_str()).unwrap_or(commit);

            // Breaking changes go under Changed (or Removed if it's a removal)
            let category = if has_breaking {
                ChangeCategory::Changed
            } else {
                ChangeCategory::from_commit_type(commit_type)
            };

            let mut desc = description.to_string();
            if has_breaking {
                desc = format!("**BREAKING:** {}", desc);
            }

            changes.push(Change {
                category,
                description: desc,
            });
        } else {
            // Fallback for commits that don't match conventional format
            changes.push(Change {
                category: ChangeCategory::Changed,
                description: commit.clone(),
            });
        }
    }

    // Sort changes by category
    changes.sort_by(|a, b| a.category.cmp(&b.category));

    // Generate changelog content
    let mut entry = format!("## [{}] - {}\n\n", version, date);

    let mut current_category: Option<ChangeCategory> = None;
    for change in changes {
        if current_category.as_ref() != Some(&change.category) {
            // Add blank line after previous list (if exists)
            if current_category.is_some() {
                entry.push('\n');
            }
            entry.push_str(&format!("{}\n\n", change.category.as_str()));
            current_category = Some(change.category);
        }
        entry.push_str(&format!("- {}\n", change.description));
    }
    // Add blank line after final list
    entry.push('\n');

    // Read existing changelog or create header
    let mut content = if changelog_path.exists() {
        fs::read_to_string(changelog_path)?
    } else {
        String::from("# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n")
    };

    // Find where to insert the new entry (after the header, before existing entries)
    let insertion_point = if let Some(pos) = content.find("\n## [") {
        pos + 1
    } else {
        content.len()
    };

    content.insert_str(insertion_point, &entry);

    // Write updated changelog
    fs::write(changelog_path, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_change_category_from_commit_type() {
        assert_eq!(
            ChangeCategory::from_commit_type("feat"),
            ChangeCategory::Added
        );
        assert_eq!(
            ChangeCategory::from_commit_type("fix"),
            ChangeCategory::Fixed
        );
        assert_eq!(
            ChangeCategory::from_commit_type("refactor"),
            ChangeCategory::Changed
        );
        assert_eq!(
            ChangeCategory::from_commit_type("perf"),
            ChangeCategory::Changed
        );
        assert_eq!(
            ChangeCategory::from_commit_type("revert"),
            ChangeCategory::Removed
        );
        assert_eq!(
            ChangeCategory::from_commit_type("security"),
            ChangeCategory::Security
        );
    }

    #[test]
    fn test_change_category_ordering() {
        assert!(ChangeCategory::Added < ChangeCategory::Changed);
        assert!(ChangeCategory::Fixed < ChangeCategory::Security);
        assert!(ChangeCategory::Added < ChangeCategory::Fixed);
    }

    #[test]
    fn test_generate_changelog_entry_creates_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "feat: add new feature".to_string(),
            "fix: resolve bug".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        assert!(changelog_path.exists());

        let content = fs::read_to_string(&changelog_path).unwrap();
        assert!(content.contains("# Changelog"));
        assert!(content.contains("## [1.0.0]"));
        assert!(content.contains("### Added"));
        assert!(content.contains("- add new feature"));
        assert!(content.contains("### Fixed"));
        assert!(content.contains("- resolve bug"));
    }

    #[test]
    fn test_generate_changelog_entry_updates_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        // Create initial changelog
        let version1 = Version::parse("1.0.0").unwrap();
        let commits1 = vec!["feat: initial feature".to_string()];
        generate_changelog_entry_at_path(&version1, &commits1, &changelog_path).unwrap();

        // Add second version
        let version2 = Version::parse("1.1.0").unwrap();
        let commits2 = vec!["feat: another feature".to_string()];
        generate_changelog_entry_at_path(&version2, &commits2, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        // Check both versions exist and in correct order
        let v1_pos = content.find("## [1.0.0]").unwrap();
        let v2_pos = content.find("## [1.1.0]").unwrap();
        assert!(v2_pos < v1_pos, "Newer version should appear first");

        assert!(content.contains("- initial feature"));
        assert!(content.contains("- another feature"));
    }

    #[test]
    fn test_generate_changelog_entry_with_breaking_changes() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("2.0.0").unwrap();
        let commits = vec![
            "feat!: breaking change".to_string(),
            "fix: normal fix".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        assert!(content.contains("### Changed"));
        assert!(content.contains("**BREAKING:** breaking change"));
        assert!(content.contains("### Fixed"));
        assert!(content.contains("- normal fix"));
    }

    #[test]
    fn test_generate_changelog_entry_with_scopes() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "feat(api): add new endpoint".to_string(),
            "fix(ui): correct button alignment".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        assert!(content.contains("- add new endpoint"));
        assert!(content.contains("- correct button alignment"));
    }

    #[test]
    fn test_generate_changelog_entry_skips_version_bump_commits() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "feat: add feature".to_string(),
            "chore: bump version to 0.9.0".to_string(),
            "chore: sync package version".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        assert!(content.contains("- add feature"));
        assert!(!content.contains("bump version"));
        assert!(!content.contains("sync package"));
    }

    #[test]
    fn test_generate_changelog_entry_groups_by_category() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "fix: bug 1".to_string(),
            "feat: feature 1".to_string(),
            "fix: bug 2".to_string(),
            "feat: feature 2".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        // Check that features are grouped together
        let added_pos = content.find("### Added").unwrap();
        let feature1_pos = content.find("- feature 1").unwrap();
        let feature2_pos = content.find("- feature 2").unwrap();

        assert!(added_pos < feature1_pos);
        assert!(added_pos < feature2_pos);

        // Check that fixes are grouped together
        let fixed_pos = content.find("### Fixed").unwrap();
        let bug1_pos = content.find("- bug 1").unwrap();
        let bug2_pos = content.find("- bug 2").unwrap();

        assert!(fixed_pos < bug1_pos);
        assert!(fixed_pos < bug2_pos);
    }

    #[test]
    fn test_generate_changelog_entry_with_non_conventional_commits() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "feat: proper feature".to_string(),
            "Some random commit message".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        // Non-conventional commits should still be included under Changed
        assert!(content.contains("### Added"));
        assert!(content.contains("- proper feature"));
        assert!(content.contains("### Changed"));
        assert!(content.contains("Some random commit message"));
    }

    #[test]
    fn test_generate_changelog_entry_multiple_categories() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "feat: new feature".to_string(),
            "fix: bug fix".to_string(),
            "perf: performance improvement".to_string(),
            "refactor: code refactor".to_string(),
            "revert: revert change".to_string(),
            "security: security fix".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        // Verify all categories appear in the correct order
        assert!(content.contains("### Added"));
        assert!(content.contains("### Changed"));
        assert!(content.contains("### Removed"));
        assert!(content.contains("### Fixed"));
        assert!(content.contains("### Security"));

        // Verify category ordering (Added should come before Changed, etc.)
        let added_pos = content.find("### Added").unwrap();
        let changed_pos = content.find("### Changed").unwrap();
        let removed_pos = content.find("### Removed").unwrap();
        let fixed_pos = content.find("### Fixed").unwrap();
        let security_pos = content.find("### Security").unwrap();

        assert!(added_pos < changed_pos);
        assert!(changed_pos < removed_pos);
        assert!(removed_pos < fixed_pos);
        assert!(fixed_pos < security_pos);
    }

    #[test]
    fn test_generate_changelog_entry_markdown_lint_compliance() {
        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "feat: add new feature".to_string(),
            "fix: resolve bug".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        let content = fs::read_to_string(&changelog_path).unwrap();

        // Test MD032: Lists should be surrounded by blank lines
        // Check that there's a blank line after "### Added" before the list
        assert!(content.contains("### Added\n\n- add new feature\n\n"));

        // Test MD022: Headings should be surrounded by blank lines
        // Check that there's a blank line before "### Fixed" heading
        assert!(content.contains("\n\n### Fixed\n\n"));

        // Verify the list ends with a blank line
        assert!(content.contains("- resolve bug\n\n"));

        // Test: No triple newlines (double blank lines) should exist
        assert!(
            !content.contains("\n\n\n"),
            "Found triple newlines (double blank lines)"
        );

        // Test: No trailing whitespace after newlines
        for line in content.lines() {
            assert!(
                !line.ends_with(' '),
                "Found trailing whitespace on line: {}",
                line
            );
        }
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored --nocapture (automatically run in CI)
    fn test_markdown_linter_if_available() {
        use std::process::Command;

        let temp_dir = TempDir::new().unwrap();
        let changelog_path = temp_dir.path().join("CHANGELOG.md");

        let version = Version::parse("1.0.0").unwrap();
        let commits = vec![
            "feat: add new feature".to_string(),
            "fix: resolve bug".to_string(),
            "refactor: improve code".to_string(),
        ];

        generate_changelog_entry_at_path(&version, &commits, &changelog_path).unwrap();

        // Try to run markdownlint-cli if available
        // Install with: npm install -g markdownlint-cli
        let result = Command::new("markdownlint")
            .arg(changelog_path.to_str().unwrap())
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    println!("✓ Markdown linter passed!");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    panic!(
                        "Markdown linter failed!\nstdout: {}\nstderr: {}",
                        stdout, stderr
                    );
                }
            }
            Err(e) => {
                println!("⚠ markdownlint not found (optional): {}", e);
                println!("  To enable this test, install: npm install -g markdownlint-cli");
            }
        }
    }
}
