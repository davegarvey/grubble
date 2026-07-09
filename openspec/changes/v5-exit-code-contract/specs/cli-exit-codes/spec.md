## ADDED Requirements

### Requirement: Normal run exits 0 on success
The `grubble` CLI SHALL exit with code 0 when the program completes successfully, regardless of whether a version bump was performed.

#### Scenario: Successful bump
- **WHEN** `grubble` runs and a version bump is required
- **THEN** the program exits with code 0

#### Scenario: Clean no-op
- **WHEN** `grubble` runs and no version bump is required (no commits since the last tag, or no triggering commits)
- **THEN** the program exits with code 0
- **AND** prints a message indicating no bump is required

#### Scenario: Successful bump with push and tag
- **WHEN** `grubble --push --tag` runs and a version bump is performed
- **THEN** the program exits with code 0

### Requirement: Non-zero exit on error
The `grubble` CLI SHALL exit with a non-zero code when an error occurs (git failure, missing file, invalid version, etc.) and SHALL write a description of the error to stderr.

#### Scenario: Git command failure
- **WHEN** `grubble` runs and an underlying git command fails
- **THEN** the program exits with a non-zero code
- **AND** the error description is written to stderr

#### Scenario: Missing package file
- **WHEN** `grubble --preset rust` runs in a directory without a `Cargo.toml`
- **THEN** the program exits with a non-zero code
- **AND** the error description is written to stderr

### Requirement: `--bump-type` exits 0 in all non-error cases
The `grubble --bump-type` flag SHALL exit with code 0 in every case where the program completes successfully, including when no bump is needed. The bump type (`major`, `minor`, `patch`, or `none`) is communicated via stdout only.

#### Scenario: No commits since tag
- **WHEN** `grubble --bump-type` runs with no commits since the last tag
- **THEN** the program exits with code 0
- **AND** prints `none` to stdout

#### Scenario: Feature commit present
- **WHEN** `grubble --bump-type` runs with a `feat:` commit since the last tag
- **THEN** the program exits with code 0
- **AND** prints `minor` to stdout

### Requirement: `--dry-run` exits 0 on success
The `grubble --dry-run` flag SHALL exit with code 0 whenever the program completes successfully. The "would a bump happen?" signal is available via `grubble --bump-type`, not via the exit code of `--dry-run`.

#### Scenario: Bump would occur
- **WHEN** `grubble --dry-run` runs and a bump would be needed
- **THEN** the program exits with code 0
- **AND** no files are modified, no commits are created, and no tags are created

#### Scenario: No bump would occur
- **WHEN** `grubble --dry-run` runs and no bump would be needed
- **THEN** the program exits with code 0
- **AND** no files are modified, no commits are created, and no tags are created

#### Scenario: Caller wants to gate on bump type
- **WHEN** a caller wants to know whether a bump would occur
- **THEN** the caller SHALL use `grubble --bump-type` and check its stdout
- **AND** SHALL NOT rely on the exit code of `grubble --dry-run` for that signal
