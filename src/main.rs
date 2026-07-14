use clap::Parser;
use std::process;

mod analyser;
mod changelog;
mod config;
mod error;
mod git;
mod output;
mod strategy;
mod versioner;

use analyser::{analyse_commits, BumpType};
use config::Config;
use error::{BumperError, BumperResult};
use output::Output;
use strategy::load_strategy;

#[derive(Debug)]
enum ExitCode {
    Ok,
    NoBump,
}

#[derive(Parser, Debug)]
#[command(name = "grubble")]
#[command(
    about = "Automatic semantic versioning based on conventional commits",
    long_about = "Grubble - Automatic Semantic Versioning

Grubble analyzes conventional commits since the last version tag and automatically
bumps the semantic version accordingly.

Version Bump Rules:
  - feat:        -> minor bump
  - fix:         -> patch bump
  - feat!: / !:  -> major bump (breaking change)

Common Usage:
  grubble                    # Analyze and bump version
  grubble --push --tag       # Bump and push to remote with tag
  grubble --dry-run          # Check if bump is needed (exit 0 if yes, 1 if no)
  grubble --bump-type        # Output bump type (major/minor/patch/none)
  grubble --preset rust      # Use Rust Cargo.toml versioning
  grubble --changelog        # Generate CHANGELOG.md"
)]
struct Args {
    /// Push changes to remote
    #[arg(short, long)]
    push: bool,

    /// Push to this branch instead of HEAD (e.g., release/v0.35.0).
    /// When set, uses `git push --set-upstream origin <branch>`.
    /// The branch is created locally if it doesn't exist.
    #[arg(long, default_value = "")]
    git_branch: String,

    /// Suppress commit list output
    #[arg(short, long)]
    quiet: bool,

    /// Create git tag for the version
    #[arg(short, long)]
    tag: bool,

    /// Include release notes in the git tag annotation
    #[arg(short = 'r', long)]
    release_notes: bool,

    /// Output only the new version string (dry run, no changes)
    #[arg(long)]
    raw: bool,

    /// Versioning strategy (node, rust, git)
    #[arg(long)]
    preset: Option<String>,

    /// Prefix for git tags (default: v)
    #[arg(long)]
    tag_prefix: Option<String>,

    /// Prefix for commit messages
    #[arg(long)]
    commit_prefix: Option<String>,

    /// Comma-separated list of files to update (for node/rust preset)
    #[arg(long)]
    package_files: Option<String>,

    /// Git user name for commits
    #[arg(long)]
    git_user_name: Option<String>,

    /// Git user email for commits
    #[arg(long)]
    git_user_email: Option<String>,

    /// Update major version tag (e.g., v4 -> v4.x.x)
    #[arg(long)]
    update_major_tag: bool,

    /// Update minor version tag (e.g., v4.1 -> v4.1.x)
    #[arg(long)]
    update_minor_tag: bool,

    /// Generate and maintain a CHANGELOG.md file
    #[arg(long)]
    changelog: bool,

    /// Output the bump type (major, minor, patch, or none) and exit
    #[arg(long)]
    bump_type: bool,

    /// Check if version bump is needed (exit 0 if bump needed, exit 1 if no bump)
    /// Does not modify any files or create commits/tags
    #[arg(long)]
    dry_run: bool,

    /// Output format. Only valid with --bump-type, --raw, or --release-from-pr.
    #[arg(long, value_enum, default_value_t = Output::Text)]
    output: Output,

    /// Resolve a merged release PR to a tag spec for post-merge release
    /// automation. Given a PR number, fetches the PR via the GitHub API
    /// and emits the version, tag name, major tag name, merge commit SHA,
    /// and PR body. Requires `GH_TOKEN` or `GITHUB_TOKEN` in the env.
    #[arg(long, value_name = "PR_NUMBER", conflicts_with_all = ["bump_type", "raw", "dry_run"])]
    release_from_pr: Option<u32>,
}

fn log(msg: &str, is_raw: bool) {
    if !is_raw {
        println!("{}", msg);
    }
}

fn emit_raw(version: &versioner::Version, preset: &str, output: Output) {
    if output == Output::Json {
        let json = serde_json::json!({
            "version": version.to_string(),
            "preset": preset,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&json).expect("failed to serialize raw JSON output")
        );
    } else {
        println!("{}", version);
    }
}

fn run() -> BumperResult<ExitCode> {
    let args = Args::parse();

    let is_bump_type = args.bump_type;
    let is_dry_run = args.dry_run;
    let is_raw = args.raw;
    let output = args.output;

    // Handle --release-from-pr: resolves a merged release PR to a tag spec.
    // This is used by the version workflow after a release PR is merged.
    if let Some(pr_number) = args.release_from_pr {
        run_release(pr_number, output)?;
        return Ok(ExitCode::Ok);
    }

    // --output json is only valid with --bump-type, --raw, or --release-from-pr
    if output == Output::Json && !is_bump_type && !is_raw {
        return Err(BumperError::InvalidConfig(
            "--output json is only valid with --bump-type, --raw, or --release-from-pr".to_string(),
        ));
    }

    // Handle --bump-type mode
    if is_bump_type {
        run_bump_type(&args, output)?;
        return Ok(ExitCode::Ok);
    }

    // Load config from file
    let mut config = Config::load();

    // Override with CLI arguments
    if let Some(preset) = args.preset {
        config.preset = preset;
    }
    if args.package_files.is_none() {
        config.package_files = match config.preset.as_str() {
            "rust" => vec!["Cargo.toml".to_string()],
            "node" => vec!["package.json".to_string()],
            "git" => vec![],
            _ => vec!["package.json".to_string()],
        };
    }
    if let Some(tag_prefix) = args.tag_prefix {
        config.tag_prefix = tag_prefix;
    }
    if let Some(commit_prefix) = args.commit_prefix {
        config.commit_prefix = commit_prefix;
    }
    if let Some(package_files) = args.package_files {
        config.package_files = package_files.split(',').map(|s| s.to_string()).collect();
    }
    if args.push {
        config.push = true;
    }
    if args.tag {
        config.tag = true;
    }
    if args.release_notes {
        config.release_notes = true;
    }
    if let Some(git_user_name) = args.git_user_name {
        config.git_user_name = git_user_name;
    }
    if let Some(git_user_email) = args.git_user_email {
        config.git_user_email = git_user_email;
    }
    if args.update_major_tag {
        config.update_major_tag = true;
    }
    if args.update_minor_tag {
        config.update_minor_tag = true;
    }
    if args.changelog {
        config.changelog = true;
    }

    let quiet = args.quiet;

    // Force settings for raw mode
    if is_raw {
        config.raw = true;
        config.push = false;
        config.tag = false;
    }

    // Force settings for dry-run mode
    if is_dry_run {
        config.raw = true;
        config.push = false;
        config.tag = false;
        config.changelog = false;
        config.release_notes = false;
    }

    if config.release_notes && !config.tag {
        log(
            "Warning: --release-notes requires --tag to be effective.",
            is_raw,
        );
    }

    // Set git config for commits
    git::set_git_config(&config.git_user_name, &config.git_user_email)?;

    let strategy = load_strategy(&config);

    let mut current_version = strategy.get_current_version()?;
    log(&format!("Current version: {}", current_version), is_raw);

    let last_tag = git::get_last_tag()?;
    log(
        &format!("Last tag: {}", last_tag.as_deref().unwrap_or("none")),
        is_raw,
    );

    let last_tag_version = git::get_last_tag_version(&config)?;

    // Validate the file/tag relationship before any work that could mask the state.
    // Read-only modes are exempt: --raw and --dry-run are for inspection.
    if !config.raw {
        if let Some(tag_ver) = last_tag_version {
            if config.preset != "git" {
                if current_version > tag_ver {
                    let file_path = config
                        .package_files
                        .first()
                        .map(String::as_str)
                        .unwrap_or("(unknown)");
                    return Err(BumperError::InvalidConfig(format!(
                        "package version {} (in {}) is ahead of latest tag v{}. \
                         Refusing to bump.\n\n\
                         To fix, align the file and tag. Either:\n  - revert {} to match the latest tag, or\n  - create the missing tag: git tag v{} && git push origin v{}",
                        current_version,
                        file_path,
                        tag_ver,
                        file_path,
                        current_version,
                        current_version
                    )));
                }
                if current_version < tag_ver {
                    log(
                        &format!(
                            "Package version {} is behind latest tag version {}, syncing...",
                            current_version, tag_ver
                        ),
                        is_raw,
                    );
                    let updated_files = strategy.update_files(&tag_ver)?;
                    if !updated_files.is_empty() {
                        git::commit_changes(
                            &format!("v{}", tag_ver),
                            &updated_files,
                            "chore: sync package version",
                        )?;
                        log(&format!("Synced package to version {}", tag_ver), is_raw);
                    }
                    current_version = tag_ver;
                }
            }
        }
    }

    let commits = git::get_commits_since_tag(last_tag.as_deref())?;

    if !quiet {
        log("Commits to analyse:", is_raw);
        for commit in &commits {
            log(&format!("  - {}", commit), is_raw);
        }
    }

    let release_notes_message = if config.release_notes && !commits.is_empty() {
        Some(
            commits
                .iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    } else {
        None
    };

    if commits.is_empty() {
        log("No commits since last tag.", is_raw);
        if is_raw {
            emit_raw(&current_version, &config.preset, output);
        }
        return Ok(ExitCode::NoBump);
    }

    let analysis = analyse_commits(&commits, &config);
    log(
        &format!("Version bump: {}", analysis.bump.as_str().to_uppercase()),
        is_raw,
    );

    if analysis.bump == BumpType::None {
        log("No version bump required.", is_raw);
        if is_raw {
            emit_raw(&current_version, &config.preset, output);
        }
        return Ok(ExitCode::NoBump);
    }

    log("Triggering commits:", is_raw);
    if !is_raw {
        for commit in &analysis.triggering_commits {
            log(&format!("  - {}", commit), is_raw);
        }
    }

    // Warn about unknown commit types
    if !analysis.unknown_commits.is_empty() && !is_raw {
        log("Warning: The following commits have unknown or unconfigured types and did not trigger a version bump:", is_raw);
        for commit in &analysis.unknown_commits {
            log(&format!("  - {}", commit), is_raw);
        }
        log("Consider configuring these types in .versionrc.json or using standard Conventional Commits types.", is_raw);
    }

    let new_version = current_version.bump(analysis.bump);

    if config.raw {
        // --raw or --dry-run — don't modify files
        emit_raw(&new_version, &config.preset, output);
        return Ok(ExitCode::Ok);
    }

    let updated_files = strategy.update_files(&new_version)?;
    log(&format!("Updated to {}", new_version), is_raw);

    // Generate changelog if enabled
    if config.changelog {
        changelog::generate_changelog_entry(&new_version, &commits, analysis.bump)?;
        log("Updated CHANGELOG.md", is_raw);
    }

    let mut all_updated_files = updated_files.clone();
    if config.changelog {
        all_updated_files.push("CHANGELOG.md".to_string());
    }

    if !all_updated_files.is_empty() {
        git::commit_changes(
            &new_version.to_string(),
            &all_updated_files,
            &config.commit_prefix,
        )?;
    }

    if config.tag {
        git::create_tag(
            &new_version.to_string(),
            &config.tag_prefix,
            release_notes_message.as_deref(),
        )?;

        // Update major/minor version tags if requested
        if config.update_major_tag || config.update_minor_tag {
            git::update_movable_tags(
                &new_version,
                &config.tag_prefix,
                config.update_major_tag,
                config.update_minor_tag,
            )?;
        }
    }

    if config.push {
        if config.update_major_tag || config.update_minor_tag {
            git::push_with_force_tags(&args.git_branch)?;
        } else {
            git::push(&args.git_branch)?;
        }
        let mut actions = vec!["Pushed changes"];
        if config.tag {
            actions.push("and tags");
        }
        log(&format!("{}.", actions.join(" ")), is_raw);
    } else {
        // Only log if we effectively did something (commit or tag)
        if !updated_files.is_empty() || config.tag {
            let mut actions = vec!["Committed"];
            if config.tag {
                actions.push("and tagged");
            }
            log(&format!("{} locally.", actions.join(" ")), is_raw);
        }
    }

    Ok(ExitCode::Ok)
}

fn run_bump_type(args: &Args, output: Output) -> BumperResult<()> {
    let mut config = Config::load();

    if let Some(preset) = &args.preset {
        config.preset = preset.clone();
    }
    if args.package_files.is_none() {
        config.package_files = match config.preset.as_str() {
            "rust" => vec!["Cargo.toml".to_string()],
            "node" => vec!["package.json".to_string()],
            "git" => vec![],
            _ => vec!["package.json".to_string()],
        };
    }
    if let Some(tag_prefix) = &args.tag_prefix {
        config.tag_prefix = tag_prefix.clone();
    }
    if let Some(package_files) = &args.package_files {
        config.package_files = package_files.split(',').map(|s| s.to_string()).collect();
    }
    if let Some(git_user_name) = &args.git_user_name {
        config.git_user_name = git_user_name.clone();
    }
    if let Some(git_user_email) = &args.git_user_email {
        config.git_user_email = git_user_email.clone();
    }

    git::set_git_config(&config.git_user_name, &config.git_user_email)?;

    let strategy = load_strategy(&config);
    let current_version = strategy.get_current_version()?;

    let last_tag = git::get_last_tag()?;
    let commits = git::get_commits_since_tag(last_tag.as_deref())?;

    if output == Output::Json {
        let analysis = analyse_commits(&commits, &config);
        let json = serde_json::json!({
            "bump_type": analysis.bump.as_str(),
            "current_version": current_version.to_string(),
            "triggering_commits": analysis.triggering_commits,
            "unknown_commits": analysis.unknown_commits,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&json).expect("failed to serialize bump-type JSON output")
        );
        return Ok(());
    }

    if commits.is_empty() {
        println!("none");
        return Ok(());
    }

    let analysis = analyse_commits(&commits, &config);
    println!("{}", analysis.bump.as_str());

    Ok(())
}

fn main() {
    match run() {
        Ok(ExitCode::Ok) => process::exit(0),
        Ok(ExitCode::NoBump) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

/// Parses a release branch name like `release/v5.2.1` into a `Version`.
/// Returns `None` if the branch name does not match the expected pattern.
fn parse_release_branch(branch: &str) -> Option<versioner::Version> {
    // Strip an optional leading "release/" prefix.
    let version_str = branch.strip_prefix("release/").unwrap_or(branch);
    // Must be just the version with no other suffix.
    let prefix = "v";
    let stripped = version_str.strip_prefix(prefix).unwrap_or(version_str);
    versioner::Version::parse(stripped).ok()
}

/// Resolves a merged release PR to a tag specification via the GitHub API.
/// Emits either JSON (when output is `Output::Json`) or human-readable text.
fn run_release(pr_number: u32, output: Output) -> BumperResult<()> {
    use std::process::Command;

    // Require a GitHub token in the env.
    let token = std::env::var("GH_TOKEN")
        .ok()
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .ok_or_else(|| {
            BumperError::InvalidConfig(
                "--release-from-pr requires GH_TOKEN or GITHUB_TOKEN in the environment"
                    .to_string(),
            )
        })?;

    // Use `gh api` to fetch the PR. We could use reqwest directly, but `gh`
    // is already a dependency in the workflow environment and avoids adding
    // a new HTTP crate.
    let output_json = Command::new("gh")
        .args([
            "api",
            "--method",
            "GET",
            &format!("/repos/{{owner}}/{{repo}}/pulls/{}", pr_number),
        ])
        .env("GH_TOKEN", &token)
        .output()
        .map_err(|e| BumperError::GitError(format!("Failed to invoke gh CLI: {}", e)))?;

    if !output_json.status.success() {
        let stderr = String::from_utf8_lossy(&output_json.stderr);
        return Err(BumperError::InvalidConfig(format!(
            "gh api failed: {}",
            stderr.trim()
        )));
    }

    let pr: serde_json::Value = serde_json::from_slice(&output_json.stdout)
        .map_err(|e| BumperError::InvalidConfig(format!("Failed to parse PR JSON: {}", e)))?;

    // Validate the PR is merged.
    let merged = pr.get("merged").and_then(|v| v.as_bool()).unwrap_or(false);
    if !merged {
        return Err(BumperError::InvalidConfig(format!(
            "PR #{} is not merged",
            pr_number
        )));
    }

    // Extract the head branch name.
    let head_ref = pr
        .get("head")
        .and_then(|h| h.get("ref"))
        .and_then(|r| r.as_str())
        .ok_or_else(|| BumperError::InvalidConfig(format!("PR #{} has no head.ref", pr_number)))?;

    // Parse the version from the head branch.
    let version = parse_release_branch(head_ref).ok_or_else(|| {
        BumperError::InvalidConfig(format!(
            "PR #{} head branch '{}' does not match release/v<semver>",
            pr_number, head_ref
        ))
    })?;

    // Extract the merge commit SHA.
    let merge_commit_sha = pr
        .get("merge_commit_sha")
        .and_then(|s| s.as_str())
        .ok_or_else(|| {
            BumperError::InvalidConfig(format!("PR #{} has no merge_commit_sha", pr_number))
        })?;

    let title = pr
        .get("title")
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string();
    let body = pr
        .get("body")
        .and_then(|b| b.as_str())
        .unwrap_or("")
        .to_string();
    let tag_name = format!("v{}", version);
    let major_tag_name = format!("v{}", version.major);

    if output == Output::Json {
        let json = serde_json::json!({
            "version": version.to_string(),
            "tag_name": tag_name,
            "major_tag_name": major_tag_name,
            "merge_commit_sha": merge_commit_sha,
            "title": title,
            "body": body,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&json).expect("failed to serialize release JSON output")
        );
    } else {
        println!("version:        {}", version);
        println!("tag_name:      {}", tag_name);
        println!("major_tag:     {}", major_tag_name);
        println!("merge_commit:  {}", merge_commit_sha);
        println!("title:         {}", title);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_release_branch_with_prefix() {
        let v = parse_release_branch("release/v5.2.1").unwrap();
        assert_eq!(v.to_string(), "5.2.1");
    }

    #[test]
    fn parse_release_branch_without_prefix() {
        let v = parse_release_branch("v5.2.1").unwrap();
        assert_eq!(v.to_string(), "5.2.1");
    }

    #[test]
    fn parse_release_branch_bare_version() {
        let v = parse_release_branch("5.2.1").unwrap();
        assert_eq!(v.to_string(), "5.2.1");
    }

    #[test]
    fn parse_release_branch_invalid() {
        assert!(parse_release_branch("feature-x").is_none());
        assert!(parse_release_branch("release/v").is_none());
        assert!(parse_release_branch("").is_none());
    }
}
