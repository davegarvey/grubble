# initial-version-flag Specification

## Purpose
Provide `--initial-version <SEMVER>` flag on the grubble CLI to declare the last released version when no git tag exists, enabling first-run bump detection and tag creation.

## Requirements

### Requirement: --initial-version provides a baseline when no tag exists
When `--initial-version <SEMVER>` is set and no semver tag is reachable from HEAD, grubble SHALL use the given version as the release baseline instead of looking for a tag. All commits from the beginning of history SHALL be analysed to determine the bump type, and version computations SHALL use the initial version as the starting point.

#### Scenario: initial version with --bump-type on a fresh repo
- **WHEN** `grubble --initial-version 0.1.0 --bump-type` runs in a repository with no tags and two commits (`fix: first`, `fix: second`)
- **THEN** stdout SHALL contain `patch`

#### Scenario: initial version with --bump-type on a repo with a feat commit
- **WHEN** `grubble --initial-version 0.0.0 --bump-type` runs in a repository with no tags and one commit (`feat: add login`)
- **THEN** stdout SHALL contain `minor`

#### Scenario: initial version with --bump-type on empty commit history
- **WHEN** `grubble --initial-version 1.0.0 --bump-type` runs in a repository with no tags and no commits since the initial commit
- **THEN** stdout SHALL contain `none`

#### Scenario: initial version with normal bump flow creates the first tag
- **WHEN** `grubble --initial-version 0.1.0 --preset rust --tag` runs in a repository with no tags, a Cargo.toml containing `version = "0.1.0"`, and a `fix: bugfix` commit
- **THEN** Cargo.toml SHALL be updated to `0.1.1`
- **AND** a git tag `v0.1.1` SHALL be created

#### Scenario: initial version with --raw outputs the bumped version
- **WHEN** `grubble --initial-version 0.5.0 --raw` runs in a repository with no tags and a `feat: add feature` commit
- **THEN** stdout SHALL contain `0.6.0`
- **AND** no files SHALL be modified

#### Scenario: initial version with --dry-run exits 0 when a bump would happen
- **WHEN** `grubble --initial-version 0.1.0 --dry-run` runs in a repository with no tags and any conventional commit
- **THEN** the process SHALL exit with code 0 (indicating a bump is needed)

### Requirement: --initial-version errors when a tag already exists
If a semver tag is reachable from HEAD when `--initial-version` is set, grubble SHALL exit with a non-zero code and an error message on stderr. The error SHALL name the existing tag and suggest using `--release-version` instead if the user needs to force a version.

#### Scenario: tag exists on HEAD
- **WHEN** `grubble --initial-version 0.1.0` runs in a repository where tag `v0.5.0` is an ancestor of HEAD
- **THEN** grubble SHALL exit with a non-zero code
- **AND** stderr SHALL contain the text `v0.5.0`
- **AND** stderr SHALL contain the text `--release-version`

### Requirement: --initial-version validates semver format
If the value passed to `--initial-version` is not a valid semver string, grubble SHALL exit with a non-zero code and an error message.

#### Scenario: invalid semver format
- **WHEN** `grubble --initial-version not-a-version --bump-type`
- **THEN** grubble SHALL exit with a non-zero code
- **AND** stderr SHALL contain an error message about invalid version format

### Requirement: --initial-version conflicts with --release-version and --release-from-pr
The `--initial-version` flag SHALL be mutually exclusive with `--release-version` and `--release-from-pr`. These are incompatible operations.

#### Scenario: --initial-version with --release-version
- **WHEN** `grubble --initial-version 0.1.0 --release-version 0.2.0`
- **THEN** grubble SHALL exit with a non-zero code
- **AND** display a usage error about conflicting flags

#### Scenario: --initial-version with --release-from-pr
- **WHEN** `grubble --initial-version 0.1.0 --release-from-pr 42`
- **THEN** grubble SHALL exit with a non-zero code
- **AND** display a usage error about conflicting flags

### Requirement: --initial-version cannot be combined with --changelog-entry
The `--changelog-entry` flag is a pure read-only operation that reads CHANGELOG.md. Combining with `--initial-version` is meaningless.

#### Scenario: --initial-version with --changelog-entry
- **WHEN** `grubble --initial-version 0.1.0 --changelog-entry`
- **THEN** grubble SHALL exit with a non-zero code
- **AND** display a usage error about conflicting flags

### Requirement: --initial-version errors when no preset and no version file
When no preset is configured (default: git preset) and no `--preset` is specified, `--initial-version` is still valid — it uses GitStrategy which gets current version from tags (or 0.0.0) and the initial version provides the bump baseline.

#### Scenario: --initial-version with git preset (or default)
- **WHEN** `grubble --initial-version 0.1.0 --bump-type` runs in a repository with no tags and a `feat:` commit
- **THEN** stdout SHALL contain `minor`
