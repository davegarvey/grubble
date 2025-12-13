# Bumper

[![CI](https://github.com/davegarvey/bumper/actions/workflows/ci.yml/badge.svg)](https://github.com/davegarvey/bumper/actions/workflows/ci.yml)
[![Release](https://github.com/davegarvey/bumper/actions/workflows/release.yml/badge.svg)](https://github.com/davegarvey/bumper/actions/workflows/release.yml)
[![Version](https://img.shields.io/github/v/release/davegarvey/bumper)](https://github.com/davegarvey/bumper/releases)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)

Automatic semantic versioning based on conventional commits, optimised for AI-generated commit messages.

## Installation

### Pre-built Binaries (Recommended)

Download from [GitHub Releases](https://github.com/davegarvey/bumper/releases):

```bash
# Linux x86_64
curl -L https://github.com/davegarvey/bumper/releases/download/v2.3.1/bumper-linux-x86_64.tar.gz | tar xz
sudo mv bumper /usr/local/bin/

# macOS Intel
curl -L https://github.com/davegarvey/bumper/releases/download/v2.3.1/bumper-macos-x86_64.tar.gz | tar xz
sudo mv bumper /usr/local/bin/

# macOS Apple Silicon
curl -L https://github.com/davegarvey/bumper/releases/download/v2.3.1/bumper-macos-aarch64.tar.gz | tar xz
sudo mv bumper /usr/local/bin/

# Windows
curl -L https://github.com/davegarvey/bumper/releases/download/v2.3.1/bumper-windows-x86_64.zip -o bumper.zip
unzip bumper.zip
# Add to PATH
```

### Cargo Install

```bash
cargo install bumper
```

### GitHub Action

```yaml
uses: davegarvey/bumper@v3.0.0
```

### From Source

```bash
git clone https://github.com/davegarvey/bumper.git
cd bumper
cargo build --release
# Binary available at target/release/bumper
```

## Usage

```bash
# Run in your project root
bumper

# Push to remote
bumper --push

# Create git tag
bumper --tag

# Raw mode (output only version, dry run)
bumper --raw

# Suppress commit list output
bumper --quiet

# With explicit options overrides
bumper --tag --tag-prefix "release-v"
bumper --commit-prefix "chore(release): bump"
bumper --preset git --tag
bumper --release-notes --tag
bumper --package-files "Cargo.toml,client/Cargo.toml"

# Show help
bumper --help
```

## Configuration

You can use a file-based configuration for Bumper instead of CLI arguments.

Create `.versionrc.json` in your project root:

```json
{
  "packageFiles": ["Cargo.toml", "client/Cargo.toml"],
  "commitPrefix": "chore: bump version",
  "tagPrefix": "v",
  "push": false,
  "tag": false,
  "preset": "rust"
}
```

### Configuration Options

- **`packageFiles`**: Array of package files to update (default: `[]`)
- **`commitPrefix`**: Prefix for version bump commits (default: `"chore: bump version"`)
- **`tagPrefix`**: Prefix for git tags (default: `"v"`)
- **`push`**: Whether to push commits/tags to remote (default: `false`)
- **`tag`**: Whether to create git tags for versions (default: `false`)
- **`preset`**: Versioning strategy to use (default: `"git"`). Options:
  - `"rust"`: Updates `Cargo.toml` version field
  - `"git"`: Tracks version via git tags only (no file updates)
  - `"node"`: Updates `package.json` version field

## Versioning Strategies

Bumper supports different versioning strategies depending on your project type:

### Rust Projects (`preset: "rust"`)

**Best for**: Rust applications and libraries

**What it does**:

- Updates the `version` field in `Cargo.toml`
- Uses semantic versioning (major.minor.patch)
- Integrates with Cargo's package management
- **Note**: `Cargo.lock` is not updated by bumper. If needed, update it separately with `cargo update` or similar commands.

**Example usage**:

```bash
bumper --preset rust --push --tag
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
bumper --preset node --push --tag
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
bumper --preset git --push --tag
```

**When to use**: Default choice for projects that don't need file-based versioning. Useful for monorepos or projects with custom versioning schemes.

### Custom Strategies

The strategy system is designed to be extensible. You can implement custom strategies for other languages or build systems by:

1. Creating a new strategy struct that implements the `Strategy` trait
2. Adding it to the strategy loader in `src/strategy.rs`
3. Using it via configuration: `"preset": "your-custom-strategy"`

This allows bumper to work with Python projects, Go modules, Docker-based versioning, or any other versioning scheme your project requires.

### Best Practices

- **Branch Protection**: Protect your main branch and require CI checks to pass
- **Conventional Commits**: Ensure all commits follow [conventional commit format](https://www.conventionalcommits.org/en/v1.0.0/)
- **Monorepos**: Use `packageFiles` array for multiple packages
- **CI Permissions**: Grant write access to contents/commits for automated releases

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
    - uses: davegarvey/bumper@v3.0.0
      with:
        push: true
        tag: true
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
    - name: Configure Git
      run: |
        git config user.name "github-actions[bot]"
        git config user.email "github-actions[bot]@users.noreply.github.com"
    - name: Install bumper
      run: cargo install bumper
    - name: Bump version and release
      run: bumper --push --tag
```

### CI Best Practices

- **Permissions**: Add `contents: write` permission for automated commits/tags
- **Branch Protection**: Require CI checks and restrict direct pushes to main
- **Testing**: Always run `cargo test` and `cargo clippy` before releasing
- **Fetch Depth**: Use `fetch-depth: 0` for complete commit history analysis

## How It Works

1. Analyzes commits since last tag
2. Determines version bump (major/minor/patch) based on conventional commits
3. Updates package files
4. Creates git commit
5. Optionally creates git tag
6. Optionally pushes to remote

## Commit Types

- `feat:` → minor bump
- `fix:`, `refactor:`, `perf:` → patch bump
- Any type with `!` or `BREAKING CHANGE` → major bump
- `docs:`, `test:`, `chore:`, `config:` → no bump

## Troubleshooting

### Common Issues

**"Author identity unknown"**

- **Solution**: Configure git identity in CI before running bumper
- **Example**: Add git config step as shown in CI workflow

**"bumper: command not found"**

- **Solution**: Install bumper before running: `cargo install bumper`
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
- Test locally with `bumper` for debugging (pushing is disabled by default)
- Run `cargo test` to verify your project setup

## For AI Users

This tool is optimised for AI-generated commit messages that follow conventional commit format. See [.github/prompts/sc.prompt.md](.github/prompts/sc.prompt.md) for an example prompt that generates commits compatible with bumper.
