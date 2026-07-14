## ADDED Requirements

### Requirement: Reject file version ahead of latest tag
The `grubble` CLI SHALL exit with a non-zero code and a descriptive error when the package file's version is strictly greater than the latest tag's version (and `preset` is not `git`). The error SHALL name both the file version and the tag version, and SHALL state at least one way to align them.

#### Scenario: Cargo.toml ahead of tag
- **WHEN** `grubble` runs with `--preset rust` in a repository where the latest tag is `v4.9.4` and `Cargo.toml` contains `version = "5.0.0"`
- **THEN** the program exits with a non-zero code
- **AND** the error message written to stderr contains the substring `5.0.0` (the file version)
- **AND** the error message written to stderr contains the substring `4.9.4` (the tag version)
- **AND** the error message written to stderr names a fix path (e.g., "revert" or "tag")

#### Scenario: package.json ahead of tag
- **WHEN** `grubble` runs with `--preset node` in a repository where the latest tag is `v2.0.0` and `package.json` contains `"version": "3.0.0"`
- **THEN** the program exits with a non-zero code
- **AND** the error message written to stderr contains the substring `3.0.0`
- **AND** the error message written to stderr contains the substring `2.0.0`

#### Scenario: No tags exist (first release)
- **WHEN** `grubble` runs in a repository with no tags and a `Cargo.toml` containing any version
- **THEN** the program does NOT fail the new check
- **AND** proceeds with the normal bump flow (first release path)

#### Scenario: File version equals tag version
- **WHEN** `grubble` runs with `--preset rust` in a repository where the latest tag is `v4.9.4` and `Cargo.toml` contains `version = "4.9.4"`
- **THEN** the program does NOT fail the new check
- **AND** proceeds with the normal bump flow (file == tag is the steady state)

#### Scenario: File version behind tag (existing sync behavior)
- **WHEN** `grubble` runs with `--preset rust` in a repository where the latest tag is `v5.0.0` and `Cargo.toml` contains `version = "4.9.4"`
- **THEN** the program does NOT fail the new check
- **AND** syncs `Cargo.toml` to `5.0.0` (preserves existing v5.0.0 behavior)

#### Scenario: git preset ignores the check
- **WHEN** `grubble` runs with `--preset git` in a repository with a tag and a stale file containing any version
- **THEN** the program does NOT fail the new check
- **AND** proceeds with the normal bump flow (git preset does not read from a file)

#### Scenario: Read-only modes skip the check
- **WHEN** `grubble --raw` or `grubble --dry-run` runs in a repository with file version ahead of tag
- **THEN** the program does NOT fail the new check
- **AND** proceeds normally (read-only modes are for inspection; failing would be hostile)
