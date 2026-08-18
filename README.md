# Grubble

[![CI](https://github.com/davegarvey/grubble/actions/workflows/ci.yml/badge.svg)](https://github.com/davegarvey/grubble/actions/workflows/ci.yml)
[![Version & Release](https://github.com/davegarvey/grubble/actions/workflows/version.yml/badge.svg)](https://github.com/davegarvey/grubble/actions/workflows/version.yml)
[![Version](https://img.shields.io/github/v/release/davegarvey/grubble)](https://github.com/davegarvey/grubble/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Automatic semantic versioning from conventional commits. Designed to be driven by AI-generated commit messages.

Grubble uses Git tags as the version source by default. The built-in file-backed presets are `rust`, `node`, and `python`; projects in other languages should use the default `git` preset and pass the tag-derived version to their own build or packaging scripts.

## Why

Grubble reads your commit history, applies conventional commit rules, and bumps the version in one command:

- `feat:` → minor, `fix:` → patch, `!` / `BREAKING CHANGE` → major
- Uses `vX.Y.Z` Git tags as the default version source
- Optionally writes the new version to `Cargo.toml` or `package.json`
- Optionally generates a `CHANGELOG.md` (Keep a Changelog format)
- Cuts and pushes tags, including floating `v4` / `v4.1` tags for GitHub Actions
- Plays well with AI agents that emit conventional commits — see [`.github/prompts/sc.prompt.md`](.github/prompts/sc.prompt.md)

## Quick Start

```bash
# Install
cargo install grubble

# Make some conventional commits
git commit -m "feat: add login"
git commit -m "fix: handle empty input"

# Preview the next version
grubble --dry-run

# Release
grubble --push --tag
```

For CI on a protected branch, see the [release-please flow](docs/release-workflow.md).

## Installation

### Pre-built binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/davegarvey/grubble/releases):

```bash
# Linux x86_64
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-linux-x86_64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# Linux ARM64
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-linux-aarch64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# macOS Intel
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-macos-x86_64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# macOS Apple Silicon
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-macos-aarch64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri https://github.com/davegarvey/grubble/releases/latest/download/grubble-windows-x86_64.zip -OutFile grubble.zip
Expand-Archive grubble.zip
```

### Cargo

```bash
cargo install grubble
```

### From source

```bash
git clone https://github.com/davegarvey/grubble.git
cd grubble
cargo build --release
# Binary at target/release/grubble
```

### GitHub Action

```yaml
- uses: davegarvey/grubble@v5
```

The Action exposes three outputs:

| Output | Description |
| --- | --- |
| `version` | The new version (e.g. `1.2.3`). |
| `previous-version` | The version before the bump. |
| `bump-type` | One of `major`, `minor`, `patch`, `none`. |

When no bump is needed, the Action exits cleanly with `bump-type=none`.

## Usage

```bash
grubble                         # bump based on commits since the last tag
grubble --push                  # push the bump commit
grubble --tag                   # create a git tag
grubble --changelog             # generate or update CHANGELOG.md
grubble --dry-run               # preview the bump without applying it
grubble --bump-type             # print major | minor | patch | none
grubble --changelog-entry       # print the latest CHANGELOG entry
grubble --release-from-pr 79    # resolve a merged release PR to a tag spec
```

Full flag reference in [docs/cli.md](docs/cli.md).

Configuration in [docs/configuration.md](docs/configuration.md).

## Quick Reference

| Topic | Where to look |
| --- | --- |
| CLI flags | [docs/cli.md](docs/cli.md) |
| Configuration | [docs/configuration.md](docs/configuration.md) |
| Release workflow | [docs/release-workflow.md](docs/release-workflow.md) |
| Best practices | [docs/best-practices.md](docs/best-practices.md) |
| CI/CD patterns | [docs/ci-cd-patterns.md](docs/ci-cd-patterns.md) |
| Migration from v4 | [docs/migration-v4.md](docs/migration-v4.md) |
| Troubleshooting | [docs/troubleshooting.md](docs/troubleshooting.md) |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, commit guidelines, and the release process. When updating documentation, place new pages in `docs/`.

## License

[MIT](LICENSE).

## For AI Users

This tool is built for AI-generated commits that follow the conventional commit format. See [`.github/prompts/sc.prompt.md`](.github/prompts/sc.prompt.md) for a prompt that produces commits compatible with grubble.
