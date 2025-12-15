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
    match run_git_command(&["describe", "--tags", "--abbrev=0"]) {
        Ok(tag) if !tag.is_empty() => Ok(Some(tag)),
        Ok(_) => Ok(None),
        Err(_) => Ok(None), // No tags exist yet
    }
}

pub fn get_last_tag_version(config: &Config) -> BumperResult<Option<Version>> {
    let last_tag = get_last_tag()?;

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
    // Only set if not already configured locally
    if run_git_command(&["config", "--local", "user.name"]).is_err() {
        run_git_command(&["config", "user.name", user_name])?;
    }
    if run_git_command(&["config", "--local", "user.email"]).is_err() {
        run_git_command(&["config", "user.email", user_email])?;
    }
    Ok(())
}

pub fn push() -> BumperResult<()> {
    run_git_command(&["push"])?;
    run_git_command(&["push", "--tags"])?;
    Ok(())
}
