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

The project uses Husky for pre-commit hooks. Install dependencies and set up hooks:

```bash
npm install
```

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
│   └── node.rs
└── error.rs         # Error types
```

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
