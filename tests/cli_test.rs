use std::process::Command;
use tempfile::TempDir;

fn get_grubble_bin() -> String {
    // Prefer local build over globally installed grubble
    let cargo_target = std::env::var("CARGO_MANIFEST_DIR")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("target")
        .join("debug")
        .join("grubble");

    if cargo_target.exists() {
        return cargo_target.to_string_lossy().to_string();
    }

    // Try release
    let cargo_target_release = std::env::var("CARGO_MANIFEST_DIR")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("target")
        .join("release")
        .join("grubble");

    if cargo_target_release.exists() {
        return cargo_target_release.to_string_lossy().to_string();
    }

    // Fall back to PATH
    if let Ok(output) = Command::new("which").arg("grubble").output() {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    panic!("Could not find grubble binary. Build with 'cargo build' first.");
}

fn setup_test_repo() -> (TempDir, Command) {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to init git");

    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to set git email");

    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to set git name");

    // Create initial commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "chore: initial commit"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to create initial commit");

    // Create initial tag
    Command::new("git")
        .args(["tag", "v1.0.0"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to create tag");

    let mut cmd = Command::new(get_grubble_bin());
    cmd.current_dir(&temp_dir);

    (temp_dir, cmd)
}

#[test]
fn test_bump_type_no_commits() {
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "none");
}

#[test]
fn test_bump_type_patch() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a fix commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "patch");
}

#[test]
fn test_bump_type_minor() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a feat commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: add new feature"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "minor");
}

#[test]
fn test_bump_type_major() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a breaking change commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat!: breaking change"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create breaking commit");

    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "major");
}

#[test]
fn test_bump_type_no_tag_scans_all_commits() {
    let (dir, mut cmd) = setup_test_repo();

    // Delete the tag so there's no tag reachable from HEAD
    Command::new("git")
        .args(["tag", "-d", "v1.0.0"])
        .current_dir(&dir)
        .output()
        .expect("Failed to delete tag");

    // Add a feat commit — scans all history and returns minor
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: add new feature"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "minor");
}

#[test]
fn test_initial_version_bump_type_patch() {
    let (dir, mut cmd) = setup_test_repo();

    // Delete the tag so no tag is reachable from HEAD
    Command::new("git")
        .args(["tag", "-d", "v1.0.0"])
        .current_dir(&dir)
        .output()
        .expect("Failed to delete tag");

    // Add a fix commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--initial-version");
    cmd.arg("0.1.0");
    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "patch");
}

#[test]
fn test_initial_version_bump_type_minor() {
    let (dir, mut cmd) = setup_test_repo();

    // Delete the tag so no tag is reachable from HEAD
    Command::new("git")
        .args(["tag", "-d", "v1.0.0"])
        .current_dir(&dir)
        .output()
        .expect("Failed to delete tag");

    // Add a feat commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: add new feature"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--initial-version");
    cmd.arg("0.2.0");
    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "minor");
}

#[test]
fn test_initial_version_normal_bump_creates_tag() {
    let (dir, mut cmd) = setup_test_repo();

    // Delete the tag so no tag is reachable from HEAD
    Command::new("git")
        .args(["tag", "-d", "v1.0.0"])
        .current_dir(&dir)
        .output()
        .expect("Failed to delete tag");

    // Add a fix commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--initial-version");
    cmd.arg("0.1.0");
    cmd.arg("--tag");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());

    // Check the tag was created
    let tags_output = Command::new("git")
        .args(["tag", "-l"])
        .current_dir(&dir)
        .output()
        .expect("Failed to list tags");

    let tags = String::from_utf8_lossy(&tags_output.stdout);
    // v1.0.0 was deleted; v0.1.1 is the new tag (0.1.0 + patch bump)
    assert!(
        tags.contains("v0.1.1"),
        "expected v0.1.1 tag, got: {}",
        tags
    );
}

#[test]
fn test_initial_version_errors_when_tag_exists() {
    let (_dir, mut cmd) = setup_test_repo();

    // Tag v1.0.0 exists (from setup_test_repo)
    cmd.arg("--initial-version");
    cmd.arg("0.1.0");
    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("v1.0.0"));
    assert!(stderr.contains("--release-version"));
}

#[test]
fn test_initial_version_with_raw() {
    let (dir, mut cmd) = setup_test_repo();

    // Delete the tag so no tag is reachable from HEAD
    Command::new("git")
        .args(["tag", "-d", "v1.0.0"])
        .current_dir(&dir)
        .output()
        .expect("Failed to delete tag");

    // Add a feat commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: add feature"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--initial-version");
    cmd.arg("0.1.0");
    cmd.arg("--raw");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "0.2.0");
}

#[test]
fn test_initial_version_invalid_semver() {
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--initial-version");
    cmd.arg("not-a-version");
    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid version format"));
}

#[test]
fn test_no_tag_proceeds_normally() {
    let (dir, mut cmd) = setup_test_repo();

    // Delete the tag so no tag is reachable
    Command::new("git")
        .args(["tag", "-d", "v1.0.0"])
        .current_dir(&dir)
        .output()
        .expect("Failed to delete tag");

    // No --initial-version, no tag — grubble scans all commits, bumps from 0.0.0
    cmd.arg("--bump-type");
    let output = cmd.output().expect("Failed to run grubble");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Only a chore: init commit exists — no bump triggered
    assert_eq!(stdout.trim(), "none");
}

#[test]
fn test_dry_run_no_bump_exit_code() {
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--dry-run");
    let output = cmd.output().expect("Failed to run grubble");

    // Exit code 0 when no bump needed (success is no-op)
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_dry_run_bump_needed_exit_code() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a fix commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--dry-run");
    let output = cmd.output().expect("Failed to run grubble");

    // Exit code 0 when bump is needed
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_dry_run_does_not_modify_files() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a fix commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    // Create a Cargo.toml to check it doesn't get modified
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"1.0.0\"\n",
    )
    .unwrap();

    cmd.arg("--dry-run");
    cmd.arg("--preset");
    cmd.arg("rust");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));

    // Check Cargo.toml was NOT modified
    let cargo_content = std::fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
    assert!(cargo_content.contains("version = \"1.0.0\""));
}

#[test]
fn test_dry_run_does_not_create_tags() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a fix commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--dry-run");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));

    // Check no new tag was created
    let tags_output = Command::new("git")
        .args(["tag", "-l"])
        .current_dir(&dir)
        .output()
        .expect("Failed to list tags");

    let tags = String::from_utf8_lossy(&tags_output.stdout);
    assert_eq!(tags.trim(), "v1.0.0"); // Only the original tag
}

#[test]
fn test_dry_run_verbose_output() {
    let (dir, mut cmd) = setup_test_repo();

    // Add commits
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: add feature"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--dry-run");
    let output = cmd.output().expect("Failed to run grubble");

    // Log messages now go to stderr so stdout stays clean for --output json
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Current version"));
    assert!(stderr.contains("Version bump"));
}

#[test]
fn test_normal_run_no_bump_exit_code() {
    // setup_test_repo already has a v1.0.0 tag and no further commits
    let (_dir, mut cmd) = setup_test_repo();

    // Default preset is git; no commits since v1.0.0 -> no bump needed
    let output = cmd.output().expect("Failed to run grubble");

    // v5 contract: success (including no-op) exits 0
    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should NOT contain "Error:" prefix on success
    assert!(!stderr.starts_with("Error:"));
}

#[test]
fn test_raw_no_further_bump_exit_code() {
    let (dir, mut cmd) = setup_test_repo();

    // Cargo.toml present so rust preset can resolve a version
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"1.0.0\"\n",
    )
    .unwrap();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("rust");
    let output = cmd.output().expect("Failed to run grubble");

    // v5 contract: --raw exits 0 when a version is produced
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "1.0.0");
}

#[test]
fn test_error_exit_code() {
    let (dir, mut cmd) = setup_test_repo();

    // No Cargo.toml and no package.json anywhere; rust preset must fail
    cmd.arg("--preset");
    cmd.arg("rust");
    // Avoid the "syncing package version" path; just request a bump that requires reading Cargo.toml
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: something"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    let output = cmd.output().expect("Failed to run grubble");

    // v5 contract: errors exit non-zero
    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Error:"),
        "expected error on stderr, got: {}",
        stderr
    );
}

#[test]
fn test_raw_with_rust_preset_reads_cargo_toml() {
    let (dir, mut cmd) = setup_test_repo();

    // Create Cargo.toml with a version that does NOT match the git tag
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("rust");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // --raw --preset rust must read from Cargo.toml, not the v1.0.0 git tag
    assert_eq!(stdout.trim(), "0.1.0");
}

#[test]
fn test_raw_with_node_preset_reads_package_json() {
    let (dir, mut cmd) = setup_test_repo();

    // Create package.json with a version that does NOT match the git tag
    std::fs::write(
        dir.path().join("package.json"),
        "{\"name\": \"demo\", \"version\": \"2.3.4\"}\n",
    )
    .unwrap();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("node");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // --raw --preset node must read from package.json, not the v1.0.0 git tag
    assert_eq!(stdout.trim(), "2.3.4");
}

#[test]
fn test_raw_with_git_preset_unchanged() {
    // Regression guard: --raw --preset git must still read from git tags
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("git");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "1.0.0");
}

#[test]
fn test_bump_type_json_output() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a feat commit
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: add thing"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--bump-type");
    cmd.arg("--output");
    cmd.arg("json");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout = stdout.trim();
    let parsed: serde_json::Value = serde_json::from_str(stdout)
        .unwrap_or_else(|e| panic!("stdout is not valid JSON '{}': {}", stdout, e));

    assert_eq!(parsed["bump_type"], "minor");
    assert!(parsed["current_version"].is_string());
    assert!(parsed["triggering_commits"].is_array());
    assert!(parsed["unknown_commits"].is_array());
}

#[test]
fn test_raw_json_output() {
    let (dir, mut cmd) = setup_test_repo();

    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"1.0.0\"\n",
    )
    .unwrap();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("rust");
    cmd.arg("--output");
    cmd.arg("json");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout = stdout.trim();
    let parsed: serde_json::Value = serde_json::from_str(stdout)
        .unwrap_or_else(|e| panic!("stdout is not valid JSON '{}': {}", stdout, e));

    assert_eq!(parsed["version"], "1.0.0");
    assert_eq!(parsed["preset"], "rust");
}

#[test]
fn test_json_output_invalid_with_dry_run() {
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--dry-run");
    cmd.arg("--output");
    cmd.arg("json");
    let output = cmd.output().expect("Failed to run grubble");

    // --dry-run forces --raw internally. With no commits since the tag,
    // grubble exits early (no bump) with exit 0 and no JSON on stdout.
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.is_empty(),
        "expected no output for no-bump dry-run, got: {}",
        stdout
    );
}

#[test]
fn test_json_output_normal_run_no_bump() {
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--output");
    cmd.arg("json");
    let output = cmd.output().expect("Failed to run grubble");

    // No commits since tag, so no bump. Exit 0, no JSON output.
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.is_empty(),
        "expected no JSON output for no-bump, got: {}",
        stdout
    );
}

#[test]
fn test_file_ahead_of_tag_fails() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a fix commit so a bump would otherwise be triggered
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: something"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    // Cargo.toml is AHEAD of the v1.0.0 tag — this is the v5.0.0 → v6.0.0 incident state
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"5.0.0\"\n",
    )
    .unwrap();

    cmd.arg("--preset");
    cmd.arg("rust");
    let output = cmd.output().expect("Failed to run grubble");

    // Must fail (not silently use the file version as the bump base)
    assert_ne!(output.status.code(), Some(0));

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Error message must name both values and reference a fix path
    assert!(
        stderr.contains("5.0.0"),
        "expected file version in error, got: {}",
        stderr
    );
    assert!(
        stderr.contains("1.0.0"),
        "expected tag version in error, got: {}",
        stderr
    );
    assert!(
        stderr.contains("revert") || stderr.contains("tag"),
        "expected fix-path keyword in error, got: {}",
        stderr
    );
}

#[test]
fn test_file_behind_tag_syncs() {
    // Regression guard: the existing sync-up behavior must still work
    let (dir, mut cmd) = setup_test_repo();

    // Cargo.toml is BEHIND the v1.0.0 tag — should sync up
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"0.5.0\"\n",
    )
    .unwrap();

    // Add a fix commit so the bump step actually runs
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: something"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--preset");
    cmd.arg("rust");
    let output = cmd.output().expect("Failed to run grubble");

    // Sync-up must succeed; the file is then bumped from the tag version
    assert_eq!(output.status.code(), Some(0));

    // After the run, the file should have been synced to 1.0.0 then bumped to 1.0.1
    let cargo_content = std::fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("1.0.1"),
        "expected file synced+bumped to 1.0.1, got: {}",
        cargo_content
    );
}

#[test]
fn test_file_ahead_of_tag_succeeds_in_raw_mode() {
    // --raw is read-only; the bump-base check must NOT fire even when
    // the file is ahead of the tag (otherwise --raw would be hostile)
    let (dir, mut cmd) = setup_test_repo();

    // Cargo.toml ahead of tag
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"5.0.0\"\n",
    )
    .unwrap();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("rust");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "5.0.0");
}

#[test]
fn test_raw_with_python_preset_reads_pyproject_toml() {
    let (dir, mut cmd) = setup_test_repo();

    // PEP 621 layout — version does NOT match the git tag
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("python");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // --raw --preset python must read from pyproject.toml, not the v1.0.0 git tag
    assert_eq!(stdout.trim(), "0.1.0");
}

#[test]
fn test_raw_with_python_preset_reads_version_constant() {
    let (dir, mut cmd) = setup_test_repo();

    // pyproject.toml with no version field + a __version__ constant in a module
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("demo.py"),
        "__version__ = \"2.3.4\"\nVERSION = \"0.0.1\"\n",
    )
    .unwrap();

    cmd.arg("--raw");
    cmd.arg("--preset");
    cmd.arg("python");
    cmd.arg("--package-files");
    cmd.arg("pyproject.toml,demo.py");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Falls back to the __version__ constant (preferred over VERSION)
    assert_eq!(stdout.trim(), "2.3.4");
}

#[test]
fn test_python_preset_bumps_pyproject_and_constant() {
    let (dir, mut cmd) = setup_test_repo();

    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("demo.py"),
        "VERSION = \"1.0.0\"\n__version__ = \"1.0.0\"\n",
    )
    .unwrap();

    // Add a fix commit so the bump step actually runs
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: resolve bug"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--preset");
    cmd.arg("python");
    cmd.arg("--package-files");
    cmd.arg("pyproject.toml,demo.py");
    cmd.arg("--tag");
    let output = cmd.output().expect("Failed to run grubble");

    assert_eq!(output.status.code(), Some(0));

    // Both the PEP 621 field and both constant styles must be bumped
    let pyproject = std::fs::read_to_string(dir.path().join("pyproject.toml")).unwrap();
    assert!(
        pyproject.contains("version = \"1.0.1\""),
        "expected pyproject.toml bumped to 1.0.1, got: {}",
        pyproject
    );
    let module = std::fs::read_to_string(dir.path().join("demo.py")).unwrap();
    assert!(
        module.contains("VERSION = \"1.0.1\"") && module.contains("__version__ = \"1.0.1\""),
        "expected constants bumped to 1.0.1, got: {}",
        module
    );

    // Tag created and bump commit uses the standard prefix
    let tags_output = Command::new("git")
        .args(["tag", "-l"])
        .current_dir(&dir)
        .output()
        .expect("Failed to list tags");
    let tags = String::from_utf8_lossy(&tags_output.stdout);
    assert!(
        tags.contains("v1.0.1"),
        "expected v1.0.1 tag, got: {}",
        tags
    );

    let log_output = Command::new("git")
        .args(["log", "--oneline", "-1"])
        .current_dir(&dir)
        .output()
        .expect("Failed to read git log");
    let log = String::from_utf8_lossy(&log_output.stdout);
    assert!(
        log.contains("chore: bump version to 1.0.1"),
        "expected standard bump commit message, got: {}",
        log
    );
}

#[test]
fn test_python_file_behind_tag_syncs() {
    // The sync-to-tag path must apply to the python preset like rust/node
    let (dir, mut cmd) = setup_test_repo();

    // pyproject.toml is BEHIND the v1.0.0 tag — should sync up
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nversion = \"0.5.0\"\n",
    )
    .unwrap();

    // Add a fix commit so the bump step actually runs
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: something"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--preset");
    cmd.arg("python");
    let output = cmd.output().expect("Failed to run grubble");

    // Sync-up must succeed; the file is then bumped from the tag version
    assert_eq!(output.status.code(), Some(0));

    let pyproject = std::fs::read_to_string(dir.path().join("pyproject.toml")).unwrap();
    assert!(
        pyproject.contains("1.0.1"),
        "expected file synced+bumped to 1.0.1, got: {}",
        pyproject
    );
}

#[test]
fn test_python_file_ahead_of_tag_fails() {
    let (dir, mut cmd) = setup_test_repo();

    // Add a fix commit so a bump would otherwise be triggered
    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: something"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    // pyproject.toml is AHEAD of the v1.0.0 tag
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nversion = \"5.0.0\"\n",
    )
    .unwrap();

    cmd.arg("--preset");
    cmd.arg("python");
    let output = cmd.output().expect("Failed to run grubble");

    // Must fail (not silently use the file version as the bump base)
    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("5.0.0"),
        "expected file version in error, got: {}",
        stderr
    );
}

/// Set up a test repo that has a local bare "remote" added as origin.
/// Returns (work_dir, remote_dir, grubble_command) where the grubble command
/// is pre-configured to run in work_dir.
fn setup_test_repo_with_remote() -> (TempDir, TempDir, Command) {
    let remote_dir = TempDir::new().unwrap();
    let work_dir = TempDir::new().unwrap();

    Command::new("git")
        .args(["init", "--bare", "--initial-branch=main"])
        .current_dir(&remote_dir)
        .output()
        .expect("Failed to init bare remote");

    Command::new("git")
        .args(["init", "--initial-branch=main"])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to init working repo");

    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to set git email");

    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to set git name");

    let remote_path = remote_dir.path().to_str().unwrap();
    Command::new("git")
        .args(["remote", "add", "origin", remote_path])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to add remote");

    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "chore: initial commit"])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to create initial commit");

    Command::new("git")
        .args(["tag", "v1.0.0"])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to create tag");

    let mut cmd = Command::new(get_grubble_bin());
    cmd.current_dir(&work_dir);

    (work_dir, remote_dir, cmd)
}

#[test]
fn test_force_push_requires_git_branch() {
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--force-push");
    cmd.arg("--push");
    let output = cmd.output().expect("Failed to run grubble");

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--git-branch"),
        "expected clap validation error naming --git-branch on stderr, got: {}",
        stderr
    );
}

#[test]
fn test_force_push_without_push_succeeds_arg_layer() {
    // --force-push only modifies push behavior; without --push there's
    // nothing to force-push, so the bump should proceed normally.
    let (dir, mut cmd) = setup_test_repo();

    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "fix: something"])
        .current_dir(&dir)
        .output()
        .expect("Failed to create fix commit");

    cmd.arg("--force-push");
    cmd.arg("--git-branch");
    cmd.arg("release/v9.9.9");
    let output = cmd.output().expect("Failed to run grubble");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "grubble failed: {}", stderr);
}

#[test]
fn test_push_to_branch() {
    let (work_dir, remote_dir, mut cmd) = setup_test_repo_with_remote();

    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: new feature"])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--push");
    cmd.arg("--tag");
    cmd.arg("--git-branch");
    cmd.arg("release/v1.1.0");
    let output = cmd.output().expect("Failed to run grubble");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "grubble failed: {}", stderr);

    let ls_remote_output = Command::new("git")
        .args([
            "ls-remote",
            "--heads",
            remote_dir.path().to_str().unwrap(),
            "release/v1.1.0",
        ])
        .output()
        .expect("Failed to ls-remote");

    let ls_remote = String::from_utf8_lossy(&ls_remote_output.stdout);
    assert!(
        ls_remote.contains("refs/heads/release/v1.1.0"),
        "expected release/v1.1.0 on remote, got: {}",
        ls_remote
    );

    let ls_tags_output = Command::new("git")
        .args([
            "ls-remote",
            "--tags",
            remote_dir.path().to_str().unwrap(),
            "v1.1.0",
        ])
        .output()
        .expect("Failed to ls-remote tags");

    let ls_tags = String::from_utf8_lossy(&ls_tags_output.stdout);
    assert!(
        ls_tags.contains("refs/tags/v1.1.0"),
        "expected v1.1.0 tag on remote, got: {}",
        ls_tags
    );

    let ls_main_output = Command::new("git")
        .args([
            "ls-remote",
            "--heads",
            remote_dir.path().to_str().unwrap(),
            "main",
        ])
        .output()
        .expect("Failed to ls-remote main");

    let ls_main = String::from_utf8_lossy(&ls_main_output.stdout);
    assert!(
        !ls_main.contains("refs/heads/main"),
        "main should not have been pushed to, got: {}",
        ls_main
    );
}

#[test]
fn test_push_to_branch_with_force_tags() {
    let (work_dir, remote_dir, mut cmd) = setup_test_repo_with_remote();

    Command::new("git")
        .args(["commit", "--allow-empty", "-m", "feat: new feature"])
        .current_dir(&work_dir)
        .output()
        .expect("Failed to create feat commit");

    cmd.arg("--push");
    cmd.arg("--tag");
    cmd.arg("--update-minor-tag");
    cmd.arg("--git-branch");
    cmd.arg("release/v1.1");
    let output = cmd.output().expect("Failed to run grubble");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "grubble failed: {}", stderr);

    let ls_remote_output = Command::new("git")
        .args([
            "ls-remote",
            "--heads",
            remote_dir.path().to_str().unwrap(),
            "release/v1.1",
        ])
        .output()
        .expect("Failed to ls-remote");

    let ls_remote = String::from_utf8_lossy(&ls_remote_output.stdout);
    assert!(
        ls_remote.contains("refs/heads/release/v1.1"),
        "expected release/v1.1 on remote, got: {}",
        ls_remote
    );

    let ls_tags_output = Command::new("git")
        .args([
            "ls-remote",
            "--tags",
            remote_dir.path().to_str().unwrap(),
            "v1.1",
        ])
        .output()
        .expect("Failed to ls-remote tags");

    let ls_tags = String::from_utf8_lossy(&ls_tags_output.stdout);
    assert!(
        ls_tags.contains("refs/tags/v1.1"),
        "expected v1.1 minor tag on remote, got: {}",
        ls_tags
    );
}
