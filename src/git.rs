use crate::config::Config;
use crate::error::{BumperError, BumperResult};
use crate::versioner::Version;
use std::process::Command;

fn run_git_command(args: &[&str]) -> BumperResult<String> {
    let output = Command::new("git")
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes")
        .env("GIT_ASKPASS", "")
        .env("SSH_ASKPASS", "")
        .output()
        .map_err(|e| BumperError::GitError(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let message = if !stderr.is_empty() {
            stderr.to_string()
        } else if !stdout.is_empty() {
            stdout.to_string()
        } else {
            "Unknown error".to_string()
        };
        return Err(BumperError::GitError(message));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn get_last_tag() -> BumperResult<Option<String>> {
    // Only match full semver tags (vX.Y.Z), excluding floating major/minor
    // tags (v5, v4.1) that GitHub Actions uses for version references.
    let pattern = "v[0-9]*.[0-9]*.[0-9]*";
    match run_git_command(&["describe", "--tags", "--abbrev=0", "--match", pattern]) {
        Ok(tag) if !tag.is_empty() => Ok(Some(tag)),
        Ok(_) => Ok(None),
        Err(_) => Ok(None), // No tags exist yet
    }
}

/// Like get_last_tag but only considers full semver tags matching the configured
/// prefix (e.g., v5.2.0, v4.9.4). Excludes floating major/minor tags (v5, v4)
/// that GitHub Actions uses for version references but aren't valid semver.
fn get_last_semver_tag(config: &Config) -> BumperResult<Option<String>> {
    let pattern = format!("{}[0-9]*.[0-9]*.[0-9]*", config.tag_prefix);
    match run_git_command(&["describe", "--tags", "--abbrev=0", "--match", &pattern]) {
        Ok(tag) if !tag.is_empty() => Ok(Some(tag)),
        Ok(_) => Ok(None),
        Err(_) => Ok(None),
    }
}

pub fn get_last_tag_version(config: &Config) -> BumperResult<Option<Version>> {
    let last_tag = get_last_semver_tag(config)?;

    if let Some(tag) = last_tag {
        let prefix = &config.tag_prefix;
        let version_str = if tag.starts_with(prefix) {
            &tag[prefix.len()..]
        } else {
            &tag
        };

        Ok(Some(Version::parse(version_str)?))
    } else {
        Ok(None)
    }
}

pub fn get_commits_since_tag(last_tag: Option<&str>) -> BumperResult<Vec<String>> {
    let output = if let Some(tag) = last_tag {
        let range = format!("{}..HEAD", tag);
        run_git_command(&["log", &range, "--pretty=%s"])?
    } else {
        run_git_command(&["log", "--pretty=%s"])?
    };

    if output.is_empty() {
        return Ok(vec![]);
    }

    Ok(output
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

pub fn commit_changes(version: &str, files: &[String], commit_prefix: &str) -> BumperResult<()> {
    if files.is_empty() {
        return Ok(());
    }

    let mut add_args = vec!["add"];
    for file in files {
        add_args.push(file);
    }
    run_git_command(&add_args)?;

    let commit_message = format!("{} to {}", commit_prefix, version);
    run_git_command(&["commit", "-m", &commit_message])?;

    Ok(())
}

pub fn create_tag(version: &str, tag_prefix: &str, message: Option<&str>) -> BumperResult<()> {
    let tag_name = format!("{}{}", tag_prefix, version);

    if let Some(msg) = message {
        run_git_command(&["tag", "-a", &tag_name, "-m", msg])?;
    } else {
        run_git_command(&["tag", &tag_name])?;
    }

    Ok(())
}

pub fn set_git_config(user_name: &str, user_email: &str) -> BumperResult<()> {
    if !user_name.is_empty() {
        run_git_command(&["config", "user.name", user_name])?;
    }
    if !user_email.is_empty() {
        run_git_command(&["config", "user.email", user_email])?;
    }
    Ok(())
}

pub fn push(branch: &str) -> BumperResult<()> {
    if !branch.is_empty() {
        ensure_local_branch(branch)?;
    }
    if branch.is_empty() {
        run_git_command(&["push"])?;
    } else {
        run_git_command(&["push", "--set-upstream", "origin", branch])?;
    }
    run_git_command(&["push", "--tags"])?;
    Ok(())
}

fn ensure_local_branch(branch: &str) -> BumperResult<()> {
    let _ = run_git_command(&["branch", branch]);
    Ok(())
}

/// Push commits to a named branch, optionally with --force-with-lease.
/// Uses --force-with-lease instead of plain --force to prevent overwriting
/// upstream changes that the local ref doesn't know about. This is
/// the correct choice when re-creating a workflow-owned branch from
/// a different parent commit (e.g., when a release branch needs to
/// be rebased to a new main HEAD).
///
/// # Panics
/// Panics if `branch` is empty — callers must validate the branch name
/// before calling. This is enforced at the CLI arg layer by the
/// `--force-push` / `--git-branch` validation.
pub fn push_branch(branch: &str, force_with_lease: bool) -> BumperResult<()> {
    assert!(
        !branch.is_empty(),
        "push_branch requires a non-empty branch name"
    );
    ensure_local_branch(branch)?;
    let mut args = vec!["push"];
    if force_with_lease {
        args.push("--force-with-lease");
    }
    args.push("--set-upstream");
    args.push("origin");
    args.push(branch);
    run_git_command(&args)?;
    run_git_command(&["push", "--tags"])?;
    Ok(())
}

/// Update major and/or minor version tags to point to the current commit.
/// Creates lightweight tags that can be force-pushed to update on remote.
/// This is useful for GitHub Actions and libraries that want users to reference
/// by major version (e.g., @v4) and automatically get the latest release.
///
/// # Arguments
/// * `version` - The semantic version being released
/// * `tag_prefix` - Prefix for tags (typically "v")
/// * `update_major` - Whether to create/update the major version tag (e.g., v4)
/// * `update_minor` - Whether to create/update the minor version tag (e.g., v4.1)
pub fn update_movable_tags(
    version: &Version,
    tag_prefix: &str,
    update_major: bool,
    update_minor: bool,
) -> BumperResult<()> {
    if update_major {
        let major_tag = format!("{}{}", tag_prefix, version.major);
        // Delete local tag if exists (ignore errors if it doesn't exist)
        let _ = run_git_command(&["tag", "-d", &major_tag]);
        // Create new lightweight tag pointing to current commit
        run_git_command(&["tag", &major_tag])?;
    }

    if update_minor {
        let minor_tag = format!("{}{}.{}", tag_prefix, version.major, version.minor);
        // Delete local tag if exists (ignore errors if it doesn't exist)
        let _ = run_git_command(&["tag", "-d", &minor_tag]);
        // Create new lightweight tag pointing to current commit
        run_git_command(&["tag", &minor_tag])?;
    }

    Ok(())
}

/// Push commits and force-push tags to remote.
/// Uses --force to allow major/minor version tags to be updated to point
/// to newer commits. This is necessary for maintaining moving tags.
///
/// # Warning
/// Force-pushing tags can affect users who have those tags checked out locally.
/// Only use this when maintaining moving major/minor version tags.
pub fn push_with_force_tags(branch: &str) -> BumperResult<()> {
    if !branch.is_empty() {
        ensure_local_branch(branch)?;
    }
    if branch.is_empty() {
        run_git_command(&["push"])?;
    } else {
        run_git_command(&["push", "--set-upstream", "origin", branch])?;
    }
    // Force push tags to update major/minor tags on remote
    run_git_command(&["push", "--tags", "--force"])?;
    Ok(())
}
