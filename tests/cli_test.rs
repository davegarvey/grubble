use std::process::Command;
use tempfile::TempDir;

fn get_grubble_bin() -> String {
    // Try to find grubble in PATH first
    if let Ok(output) = Command::new("which").arg("grubble").output() {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    // Fall back to looking in cargo target directory
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
fn test_dry_run_no_bump_exit_code() {
    let (_dir, mut cmd) = setup_test_repo();

    cmd.arg("--dry-run");
    let output = cmd.output().expect("Failed to run grubble");

    // Exit code 1 when no bump needed
    assert_eq!(output.status.code(), Some(1));
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

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should show output in verbose mode (not raw mode)
    assert!(stdout.contains("Current version"));
    assert!(stdout.contains("Version bump"));
}
