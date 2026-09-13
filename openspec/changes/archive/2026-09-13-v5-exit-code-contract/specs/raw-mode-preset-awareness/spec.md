## ADDED Requirements

### Requirement: `--raw` honors `--preset` for the version source
The `grubble --raw` flag SHALL resolve the current version using the strategy selected by `--preset`. Specifically, `--raw` MUST NOT short-circuit to a git-tag-based strategy when a different `--preset` is configured.

#### Scenario: `git` preset reads from git tags
- **WHEN** `grubble --raw --preset git` runs in a repository with a `v1.2.3` tag
- **THEN** the program prints `1.2.3` to stdout
- **AND** exits with code 0

#### Scenario: `rust` preset reads from Cargo.toml
- **WHEN** `grubble --raw --preset rust` runs in a directory with `Cargo.toml` containing `version = "0.1.0"`
- **THEN** the program prints `0.1.0` to stdout
- **AND** exits with code 0
- **AND** the value is read from `Cargo.toml`, not from any git tag

#### Scenario: `node` preset reads from package.json
- **WHEN** `grubble --raw --preset node` runs in a directory with `package.json` containing `"version": "2.3.4"`
- **THEN** the program prints `2.3.4` to stdout
- **AND** exits with code 0
- **AND** the value is read from `package.json`, not from any git tag

#### Scenario: No preset defaults to git
- **WHEN** `grubble --raw` runs in a repository with a `v1.2.3` tag and no `--preset` flag
- **THEN** the program prints `1.2.3` to stdout
- **AND** exits with code 0

### Requirement: `--raw` exits 0 when a version is produced
The `grubble --raw` flag SHALL exit with code 0 whenever a version string is successfully written to stdout, including when no further bump is needed.

#### Scenario: Raw with no commits since tag
- **WHEN** `grubble --raw --preset rust` runs in a directory with a `Cargo.toml` containing `version = "1.0.0"` and no new commits
- **THEN** the program prints `1.0.0` to stdout
- **AND** exits with code 0

#### Scenario: Raw with commits that would trigger a bump
- **WHEN** `grubble --raw --preset rust` runs in a directory with `Cargo.toml` containing `version = "1.0.0"` and a `fix:` commit
- **THEN** the program prints `1.0.1` to stdout
- **AND** exits with code 0

### Requirement: `--raw` propagates errors with non-zero exit
The `grubble --raw` flag SHALL exit with a non-zero code when the configured strategy cannot resolve a version (missing file, missing version field, invalid format) and SHALL write a description of the error to stderr.

#### Scenario: Rust preset with missing Cargo.toml
- **WHEN** `grubble --raw --preset rust` runs in a directory without a `Cargo.toml`
- **THEN** the program exits with a non-zero code
- **AND** the error description is written to stderr
- **AND** no output is written to stdout

#### Scenario: Node preset with malformed version
- **WHEN** `grubble --raw --preset node` runs in a directory with `package.json` containing an invalid version field
- **THEN** the program exits with a non-zero code
- **AND** the error description is written to stderr
