# Grubble

[![CI](https://github.com/davegarvey/grubble/actions/workflows/ci.yml/badge.svg)](https://github.com/davegarvey/grubble/actions/workflows/ci.yml)
[![Version & Release](https://github.com/davegarvey/grubble/actions/workflows/version.yml/badge.svg)](https://github.com/davegarvey/grubble/actions/workflows/version.yml)
[![Version](https://img.shields.io/github/v/release/davegarvey/grubble)](https://github.com/davegarvey/grubble/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)

Automatic semantic versioning based on conventional commits, optimised for AI-generated commit messages.

## Installation

### Pre-built Binaries (Recommended)

Download from [GitHub Releases](https://github.com/davegarvey/grubble/releases):

```bash
# Linux x86_64
curl -L https://github.com/davegarvey/grubble/releases/download/v4.0.0/grubble-linux-x86_64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# macOS Intel
curl -L https://github.com/davegarvey/grubble/releases/download/v4.0.0/grubble-macos-x86_64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# macOS Apple Silicon
curl -L https://github.com/davegarvey/grubble/releases/download/v4.0.0/grubble-macos-aarch64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# Windows
curl -L https://github.com/davegarvey/grubble/releases/download/v4.0.0/grubble-windows-x86_64.zip -o grubble.zip
unzip grubble.zip
# Add grubble.exe to PATH
```

### Cargo Install

```bash
cargo install grubble
```

### GitHub Action

```yaml
uses: davegarvey/grubble@v4.0.0
```

### From Source

```bash
git clone https://github.com/davegarvey/grubble.git
cd grubble
cargo build --release
# Binary available at target/release/grubble
```

## Usage

```bash
# Run in your project root
grubble

# Push to remote
grubble --push

# Create git tag
grubble --tag

# Raw mode (output only version, dry run)
grubble --raw

# Suppress commit list output
grubble --quiet

# With explicit options overrides
grubble --tag --tag-prefix "release-v"
grubble --commit-prefix "chore(release): bump"
grubble --preset git --tag
grubble --release-notes --tag
grubble --package-files "Cargo.toml,client/Cargo.toml"
grubble --git-user-name "My Name" --git-user-email "my@email.com"

# Update major version tag (e.g., v4 -> v4.x.x)
grubble --tag --update-major-tag

# Update both major and minor version tags
grubble --tag --update-major-tag --update-minor-tag --push

# Show help
grubble --help
```

## Configuration

Grubble can be configured using CLI arguments or a `.versionrc.json` file.

### CLI Configuration (Recommended for CI/CD)

All options can be passed as command-line arguments:

```bash
grubble \
  --package-files Cargo.toml \
  --commit-prefix "chore: bump version" \
  --tag-prefix v \
  --preset rust \
  --push \
  --tag \
  --release-notes
```

### File-based Configuration

Alternatively, create `.versionrc.json` in your project root:

```json
{
  "packageFiles": ["Cargo.toml", "client/Cargo.toml"],
  "commitPrefix": "chore: bump version",
  "tagPrefix": "v",
  "push": false,
  "tag": false,
  "preset": "rust",
  "types": {
    "config": "patch"
  }
}
```

### Configuration Options

- **`packageFiles`**: Array of package files to update (default: `[]`)
- **`commitPrefix`**: Prefix for version bump commits (default: `"chore: bump version"`)
- **`tagPrefix`**: Prefix for git tags (default: `"v"`)
- **`push`**: Whether to push commits/tags to remote (default: `false`)
- **`tag`**: Whether to create git tags for versions (default: `false`)
- **`updateMajorTag`**: Update major version tag (e.g., v4 pointing to latest v4.x.x) (default: `false`)
- **`updateMinorTag`**: Update minor version tag (e.g., v4.1 pointing to latest v4.1.x) (default: `false`)
- **`gitUserName`**: Git user name for commits (default: `"grubble-bot"`)
- **`gitUserEmail`**: Git user email for commits (default: `"grubble-bot@noreply.local"`)
  - *Note: These values are only used when no local git user.name/email configuration exists in the repository. If git config is already set locally, these values are ignored. For CI/CD environments, configure these to match your platform's bot user (e.g., GitHub Actions bot, GitLab CI bot, etc.).*
- **`preset`**: Versioning strategy to use (default: `"git"`). Options:
  - `"rust"`: Updates `Cargo.toml` version field
  - `"git"`: Tracks version via git tags only (no file updates)
  - `"node"`: Updates `package.json` version field
- **`types`**: Object mapping commit types to version bump behavior (default: see Commit Types section). Valid values: `"major"`, `"minor"`, `"patch"`, `"none"`
  - *Example*: `{"config": "patch", "revert": "none"}`

## Versioning Strategies

Grubble supports different versioning strategies depending on your project type:

### Rust Projects (`preset: "rust"`)

**Best for**: Rust applications and libraries

**What it does**:

- Updates the `version` field in `Cargo.toml`
- Automatically updates `Cargo.lock` if present (recommended for binary crates)
- Uses semantic versioning (major.minor.patch)
- Integrates with Cargo's package management

**Example usage**:

```bash
grubble --preset rust --push --tag
```

**When to use**: For Rust projects. Automatically updates your Cargo.toml and works seamlessly with `cargo publish`.

### Node.js Projects (`preset: "node"`)

**Best for**: JavaScript/TypeScript applications and packages

**What it does**:

- Updates the `version` field in `package.json`
- Updates `package-lock.json` if present
- Compatible with npm/yarn ecosystem

**Example usage**:

```bash
grubble --preset node --push --tag
```

**When to use**: For Node.js projects. Automatically updates your package.json and works seamlessly with npm/yarn publishing.

### Git-only Projects (`preset: "git"`)

**Best for**: Projects that don't need file-based versioning

**What it does**:

- Only creates git tags for versioning
- No files are modified
- Tracks versions purely through git history

**Example usage**:

```bash
grubble --preset git --push --tag
```

**When to use**: Default choice for projects that don't need file-based versioning. Useful for monorepos or projects with custom versioning schemes.

### Custom Strategies

The strategy system is designed to be extensible. You can implement custom strategies for other languages or build systems by:

1. Creating a new strategy struct that implements the `Strategy` trait
2. Adding it to the strategy loader in `src/strategy.rs`
3. Using it via configuration: `"preset": "your-custom-strategy"`

This allows grubble to work with Python projects, Go modules, Docker-based versioning, or any other versioning scheme your project requires.

### Package Version Syncing

When switching from the `git` strategy (tag-only) to file-based strategies like `node` or `rust`, or if package files are outdated compared to existing tags, Grubble automatically syncs the package versions:

- Compares the current package file version against the latest git tag
- If the package version is behind, updates the package files to match the tag version
- Commits the sync with a descriptive message (e.g., "chore: sync package version to v1.2.3")
- Then proceeds with normal versioning logic based on recent commits

This ensures version consistency across strategies and prevents conflicts when creating new tags.

## Major/Minor Version Tag Tracking

**Best for**: Maintainers of GitHub Actions, reusable workflows, or libraries where users reference by major version

### What It Does

When enabled, Grubble maintains "floating" major (and optionally minor) version tags that always point to the latest release in that version range:

- **Major tag** (e.g., `v4`) → always points to latest `v4.x.x` release
- **Minor tag** (e.g., `v4.1`) → always points to latest `v4.1.x` release

This follows GitHub Actions best practices where users can reference `uses: owner/repo@v4` to automatically get the latest v4 release without manually updating to each new patch version.

### Usage

**CLI:**

```bash
# Update major version tag only
grubble --tag --push --update-major-tag

# Update both major and minor version tags
grubble --tag --push --update-major-tag --update-minor-tag
```

**GitHub Action:**

```yaml
- uses: davegarvey/grubble@v4
  with:
    tag: true
    push: true
    update-major-tag: true
```

**Configuration file (.versionrc.json):**

```json
{
  "tag": true,
  "push": true,
  "updateMajorTag": true,
  "updateMinorTag": false
}
```

### How It Works

1. Creates the standard semantic version tag (e.g., `v4.2.3`)
2. Updates or creates the major version tag (`v4`) pointing to the same commit
3. Optionally updates the minor version tag (`v4.2`) pointing to the same commit
4. Force-pushes tags to update them on the remote

### When to Use

✅ **Use major version tracking when:**

- Publishing GitHub Actions for users to reference by major version
- Maintaining libraries where users want automatic patch updates
- Following semantic versioning with stable major version APIs

⚠️ **Consider the implications:**

- Force-pushing tags can affect users who have those tags locally
- Users pinned to major versions will automatically get updates
- Major version tags are lightweight (not annotated) by design
- Requires explicit opt-in to avoid unintended behavior

### Best Practices

- **Document clearly**: Let users know they can use major version references
- **Test thoroughly**: Ensure patch/minor updates won't break users on major version pins
- **Semantic versioning**: Only use this if you follow semver strictly (breaking changes = major bump)
- **CI/CD**: Automate this in your release workflow for consistency
- **GitHub Actions**: Essential for action maintainers to provide a good user experience

### Best Practices

- **Branch Protection**: Protect your main branch and require CI checks to pass
- **Conventional Commits**: Ensure all commits follow [conventional commit format](https://www.conventionalcommits.org/en/v1.0.0/)
- **Monorepos**: Use `packageFiles` array for multiple packages
- **CI Permissions**: Grant write access to contents/commits for automated releases

### Local Git Hooks

Run once to enable the shared hooks path:

```bash
git config core.hooksPath scripts/hooks
```

The pre-commit hook runs `cargo fmt --all` (fixes formatting) and `cargo clippy --all-targets --all-features -- -D warnings` so commits fail early if code would break CI checks. You can temporarily skip steps with `SKIP_FMT=1` or `SKIP_CLIPPY=1`, and opt into running tests with `RUN_TESTS=1`.

## GitHub Actions

### Recommended: Use GitHub Action (Simplest)

```yaml
name: Release
on:
  pull_request:
    types: [closed]
    branches: [main]

jobs:
  release:
    if: github.event.pull_request.merged == true
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: read
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    - uses: davegarvey/grubble@v3.0.0
      with:
        push: true
        tag: true
        update-major-tag: true  # Maintain v3 pointing to latest v3.x.x
```

### Alternative: Manual Setup

If you prefer more control over the process:

```yaml
name: Release
on:
  pull_request:
    types: [closed]
    branches: [main]

jobs:
  release:
    if: github.event.pull_request.merged == true
    runs-on: ubuntu-latest
    permissions:
      contents: write      # Required for pushing commits/tags
      pull-requests: read  # Required for PR info
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0  # Required for commit analysis
    - name: Setup Rust
      uses: actions-rust-lang/setup-rust-toolchain@v1
      with:
        toolchain: stable
    - name: Run tests
      run: cargo test
    - name: Run clippy
      run: cargo clippy -- -D warnings
    - name: Install grubble
      run: cargo install grubble
    - name: Bump version and release
      run: |
        grubble \
          --push \
          --tag \
          --git-user-name "github-actions[bot]" \
          --git-user-email "41898282+github-actions[bot]@users.noreply.github.com"
```

### CI Best Practices

- **Permissions**: Add `contents: write` permission for automated commits/tags
- **Branch Protection**: Require CI checks and restrict direct pushes to main
- **Testing**: Always run `cargo test` and `cargo clippy` before releasing
- **Fetch Depth**: Use `fetch-depth: 0` for complete commit history analysis

## How It Works

1. Syncs package versions if behind latest tag (for file-based strategies)
2. Analyzes commits since last tag
3. Determines version bump (major/minor/patch) based on conventional commits
4. Updates package files
5. Creates git commit
6. Optionally creates git tag
7. Optionally pushes to remote

## Commit Types

- `feat:` → minor bump
- `fix:` → patch bump
- Any type with `!` or `BREAKING CHANGE` → major bump
- `docs:`, `test:`, `chore:`, `ci:`, `build:`, `style:`, `refactor:`, `perf:` → no bump

*Note: These are the default mappings. You can customize version bump behavior for any commit type using the `types` configuration in `.versionrc.json`.*

## Troubleshooting

### Common Issues

**"Author identity unknown"**

- **Solution**: Configure git identity in CI before running grubble
- **Example**: Add git config step as shown in CI workflow

**"grubble: command not found"**

- **Solution**: Install grubble before running: `cargo install grubble`
- **Why**: Ensure the binary is in PATH or use full path

**No version bump on merge**

- **Check**: Ensure PR contains conventional commits with `feat:`, `fix:`, etc.
- **Check**: Verify CI has write permissions to repository
- **Check**: Confirm `fetch-depth: 0` in checkout action

**Invalid config file**

- **Solution**: Ensure `.versionrc.json` contains valid JSON
- **Note**: Empty or invalid files fall back to defaults with a warning

**Package file not found**

- **Solution**: Use `--package-files` to specify correct file paths for your project type
- **Check**: Verify file exists and contains valid `version` field

### Getting Help

- Check commit format with conventional commits specification
- Verify CI permissions and branch protection rules
- Test locally with `grubble` for debugging (pushing is disabled by default)
- Run `cargo test` to verify your project setup

## For AI Users

This tool is optimised for AI-generated commit messages that follow conventional commit format. See [.github/prompts/sc.prompt.md](.github/prompts/sc.prompt.md) for an example prompt that generates commits compatible with grubble.
