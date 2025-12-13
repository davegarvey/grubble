use clap::Parser;
use std::process;

mod analyser;
mod config;
mod error;
mod git;
mod strategy;
mod versioner;

// Trivial change for testing major version bump

use analyser::{analyse_commits, BumpType};
use config::Config;
use error::BumperResult;
use strategy::load_strategy;

#[derive(Parser, Debug)]
#[command(name = "bump")]
#[command(about = "Automatic semantic versioning based on conventional commits", long_about = None)]
struct Args {
    /// Push changes to remote
    #[arg(short, long)]
    push: bool,

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
}

fn log(msg: &str, is_raw: bool) {
    if !is_raw {
        println!("{}", msg);
    }
}

fn run() -> BumperResult<()> {
    let args = Args::parse();

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

    let quiet = args.quiet;

    let is_raw = args.raw;

    // Force settings for raw mode
    if is_raw {
        config.raw = true;
        config.push = false;
        config.tag = false;
    }

    if config.release_notes && !config.tag {
        log(
            "Warning: --release-notes requires --tag to be effective.",
            is_raw,
        );
    }

    let strategy = load_strategy(&config);

    let current_version = strategy.get_current_version()?;
    log(&format!("Current version: {}", current_version), is_raw);

    let last_tag = git::get_last_tag()?;
    log(
        &format!("Last tag: {}", last_tag.as_deref().unwrap_or("none")),
        is_raw,
    );

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
            println!("{}", current_version);
        }
        return Ok(());
    }

    let analysis = analyse_commits(&commits, &config);
    log(
        &format!("Version bump: {}", analysis.bump.as_str().to_uppercase()),
        is_raw,
    );

    if analysis.bump == BumpType::None {
        log("No version bump required.", is_raw);
        if is_raw {
            println!("{}", current_version);
        }
        return Ok(());
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

    if is_raw {
        println!("{}", new_version);
        return Ok(());
    }

    let updated_files = strategy.update_files(&new_version)?;
    log(&format!("Updated to {}", new_version), is_raw);

    if !updated_files.is_empty() {
        git::commit_changes(
            &new_version.to_string(),
            &updated_files,
            &config.commit_prefix,
        )?;
    }

    if config.tag {
        git::create_tag(
            &new_version.to_string(),
            &config.tag_prefix,
            release_notes_message.as_deref(),
        )?;
    }

    if config.push {
        git::push()?;
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

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
