# Contributing to Grubble

Thank you for your interest in contributing to Grubble! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- **Rust**: Install the latest stable version from [rustup.rs](https://rustup.rs/)
- **Node.js** (optional): For markdown linting tests, install from [nodejs.org](https://nodejs.org/)

### Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/davegarvey/grubble.git
   cd grubble
   ```

2. Run tests:

   ```bash
   cargo test
   ```

3. Run linting:

   ```bash
   cargo clippy
   ```

## Testing

### Standard Test Suite

Run all tests with:

```bash
cargo test
```

### Markdown Linting Tests

Grubble includes optional tests that validate generated changelog files against markdown linting rules. These tests require `markdownlint-cli` and are controlled by the `RUN_MARKDOWN_LINT` environment variable.

#### Why Optional?

- **Dependencies**: Requires Node.js and `markdownlint-cli`
- **Performance**: External process calls are slower
- **CI Focus**: Primarily valuable in CI where standards are enforced

#### Running Markdown Linting Tests

**Option 1: Environment Variable (Recommended)**

```bash
# Set for current session
export RUN_MARKDOWN_LINT=1
cargo test

# Or inline
RUN_MARKDOWN_LINT=1 cargo test
```

**Option 2: .env File**

```bash
# Create .env file
echo 'RUN_MARKDOWN_LINT=1' > .env

# Run tests with environment loaded
export $(cat .env) && cargo test
```

**Option 3: Convenience Script**

```bash
# Use the provided script
./test-with-markdown.sh
./test-with-markdown.sh test_markdown_linter_if_available -- --nocapture
```

#### Test Behavior

- **CI Environment**: Tests run automatically when `CI=true`
- **Local Development**: Tests skip gracefully unless `RUN_MARKDOWN_LINT=1` is set
- **Missing Dependencies**: In CI, tests fail if `markdownlint-cli` is unavailable

### Test Coverage

The project maintains comprehensive test coverage including:

- Unit tests for all core functionality
- Integration tests for changelog generation
- Markdown compliance tests
- Cross-platform compatibility tests

## Code Quality

### Linting

Run clippy for code quality checks:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Formatting

Format code according to Rust standards:

```bash
cargo fmt --all
```

### Pre-commit Hooks

The project ships a plain git hook in `scripts/hooks/pre-commit`. Enable it once after cloning:

```bash
git config core.hooksPath scripts/hooks
```

The hook runs `cargo fmt --all` and `cargo clippy --all-targets --all-features -- -D warnings`, so commits fail early if code would break CI. Skip individual steps with `SKIP_FMT=1` or `SKIP_CLIPPY=1`, and opt into running tests with `RUN_TESTS=1`.

## Commit Guidelines

Grubble follows [Conventional Commits](https://conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Commit Types

- `feat:` - New features (minor version bump)
- `fix:` - Bug fixes (patch version bump)
- `docs:` - Documentation changes
- `test:` - Testing related changes
- `refactor:` - Code refactoring
- `style:` - Code style changes
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes
- `build:` - Build system changes
- `perf:` - Performance improvements

### Breaking Changes

Mark breaking changes with `!` after the type/scope or include `BREAKING CHANGE:` in the footer.

### Examples

```bash
feat: add support for custom commit types
fix: resolve changelog spacing issue
test: improve markdown linter test behavior
docs: update installation instructions
refactor!: simplify version parsing logic
```

## Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Make** your changes following the guidelines above
4. **Test** thoroughly (including markdown linting if applicable)
5. **Commit** with conventional commit messages
6. **Push** to your fork
7. **Create** a Pull Request with a clear description

### PR Requirements

- All tests pass
- Code is formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)
- Conventional commit messages
- Clear description of changes

## Release Process

Grubble uses automated releases based on conventional commits:

- `feat:` commits → Minor version bump
- `fix:` commits → Patch version bump
- Breaking changes → Major version bump

Releases are automatically created via GitHub Actions when changes are merged to `main`.

### Don't manually bump the package file in a release PR

The `Version & Release` workflow (`version.yml`) runs `grubble` after the merge, which:

1. Reads the **latest git tag** as the base version
2. Examines **commits since that tag** for conventional-commit prefixes
3. Computes the next version (e.g. `4.9.4` + `feat!:` → `5.0.0`)
4. Updates `Cargo.toml` (or `package.json`), commits, and tags

**Do not manually bump `Cargo.toml` or `package.json` in a release PR.** Manually setting the file to a version ahead of the latest tag is the "file ahead of tag" state, which `grubble` rejects with a clear error:

```
Error: Invalid configuration: package version 5.0.0 (in Cargo.toml) is 
ahead of latest tag v4.9.4. Refusing to bump.

To fix, align the file and tag. Either:
  - revert Cargo.toml to match the latest tag, or
  - create the missing tag: git tag v5.0.0 && git push origin v5.0.0
```

If you need to make a "manual" version bump (e.g. for a compliance reason before tagging), do it the way the workflow would: create the tag yourself (`git tag v5.0.0 && git push origin v5.0.0`), let the file catch up via a follow-up commit, and let the next workflow run perform the next bump from that tag.

### A note on patch releases after a breaking change

If you merge a breaking change (e.g. v5.0.0) and then later need to ship a patch (v5.0.1) that contains only `fix:` commits, the workflow will correctly produce v5.0.1 — the latest tag is v5.0.0, the commits since v5.0.0 are `fix:`, the bump is patch. Do not manually set `Cargo.toml` to `5.0.1` "to help" the workflow; this triggers the same "file ahead of tag" error.

## Project Structure

```
src/
├── main.rs          # CLI entry point
├── analyser.rs      # Commit analysis logic
├── versioner.rs     # Version bump calculations
├── changelog.rs     # CHANGELOG.md generation
├── config.rs        # Configuration handling
├── git.rs           # Git operations
├── strategy/        # Version strategy implementations
│   ├── git.rs
│   ├── node.rs
│   └── rust.rs
└── error.rs         # Error types
```

## Adding a Custom Strategy

Strategies encapsulate "where does the version live" for a project. To add one (e.g. for Python `pyproject.toml` or Go modules):

1. Add a new file in `src/strategy/` implementing the `Strategy` trait defined in `src/strategy.rs`.
2. Register it in the strategy loader in `src/strategy.rs`.
3. Reference it from configuration with `"preset": "your-preset"`.

Use the existing `rust.rs` and `node.rs` strategies as references — they are thin wrappers around read/update of a single version field.

## Configuration Files

- `.versionrc.json` - Version bump configuration
- `.markdownlint.json` - Markdown linting rules
- `.github/workflows/` - CI/CD pipelines

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/davegarvey/grubble/issues)
- **Discussions**: [GitHub Discussions](https://github.com/davegarvey/grubble/discussions)
- **Documentation**: See README.md for detailed usage instructions

## License

By contributing to Grubble, you agree that your contributions will be licensed under the same MIT License that covers the project.
