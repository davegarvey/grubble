# machine-readable-output Specification

## Purpose

Defines stable text and JSON output contracts for integrating grubble with automation and CI/CD tooling.

## Requirements

### Requirement: --output flag controls output format
The `grubble` CLI SHALL accept an `--output` flag with values `text` (default) and `json`. The flag applies to bump, `--bump-type`, `--raw`, and `--release-from-pr` modes. When `--output` is set to any value other than `text` or `json`, the program SHALL exit with a non-zero code and a clear error.

#### Scenario: Default text output
- **WHEN** `grubble --bump-type` runs without `--output`
- **THEN** the output format SHALL be `text`
- **AND** the program SHALL print `major`, `minor`, `patch`, or `none` to stdout

#### Scenario: Explicit text output
- **WHEN** `grubble --bump-type --output text` runs
- **THEN** the program SHALL print `major`, `minor`, `patch`, or `none` to stdout

#### Scenario: Invalid output value
- **WHEN** `grubble --bump-type --output yaml` runs
- **THEN** the program SHALL exit with a non-zero code
- **AND** a clear error message naming the valid values SHALL be written to stderr

### Requirement: --bump-type --output json schema
The `grubble --bump-type --output json` mode SHALL write a single JSON object to stdout with the following structure:
```json
{
  "bump_type": "major | minor | patch | none",
  "current_version": "<string>",
  "triggering_commits": ["<string>"],
  "unknown_commits": ["<string>"]
}
```

The JSON object MUST be the only content written to stdout in this mode.

#### Scenario: JSON output for a minor bump
- **WHEN** `grubble --bump-type --output json` runs against a repo with a `feat:` commit
- **THEN** stdout SHALL contain a single JSON object
- **AND** `bump_type` SHALL be `"minor"`
- **AND** `current_version` SHALL be the parsed current version string
- **AND** `triggering_commits` SHALL be a non-empty array containing the triggering commit
- **AND** `unknown_commits` SHALL be an empty array or absent

#### Scenario: JSON output for no bump
- **WHEN** `grubble --bump-type --output json` runs with no triggering commits
- **THEN** stdout SHALL contain a single JSON object
- **AND** `bump_type` SHALL be `"none"`
- **AND** `current_version` SHALL be the parsed current version string
- **AND** `triggering_commits` SHALL be an empty array
- **AND** `unknown_commits` SHALL contain any unrecognized commit messages

#### Scenario: JSON output for a major bump
- **WHEN** `grubble --bump-type --output json` runs against a repo with a `feat!:` commit
- **THEN** stdout SHALL contain a single JSON object
- **AND** `bump_type` SHALL be `"major"`

### Requirement: --raw --output json schema
The `grubble --raw --output json` mode SHALL write a single JSON object to stdout with the following structure:
```json
{
  "version": "<string>",
  "previous_version": "<string>",
  "bump_type": "major | minor | patch | none",
  "preset": "<string>"
}
```

`version` is the predicted version, `previous_version` is the version the bump would start from, and `bump_type` is the bump that produces `version`. When no bump is needed, `version` and `previous_version` are both the current version and `bump_type` is `"none"`.

The JSON object MUST be the only content written to stdout in this mode.

#### Scenario: JSON raw output
- **WHEN** `grubble --raw --output json --preset rust` runs in a directory with `Cargo.toml` containing `version = "1.2.3"`
- **THEN** stdout SHALL contain a single JSON object
- **AND** `version` SHALL be `"1.2.3"`
- **AND** `preset` SHALL be `"rust"`

#### Scenario: JSON raw output with default preset
- **WHEN** `grubble --raw --output json` runs in a git repo with a `v1.2.3` tag and no `--preset`
- **THEN** stdout SHALL contain a single JSON object
- **AND** `version` SHALL be `"1.2.3"`
- **AND** `preset` SHALL be `"git"`

#### Scenario: JSON raw output with a pending bump
- **WHEN** `grubble --raw --output json` runs in a git repo with a `v1.0.0` tag followed by a `fix:` commit
- **THEN** `version` SHALL be `"1.0.1"`
- **AND** `previous_version` SHALL be `"1.0.0"`
- **AND** `bump_type` SHALL be `"patch"`

### Requirement: JSON output supports normal bump and dry-run modes
The `grubble` CLI SHALL accept `--output json` in normal bump and dry-run modes. A successful version bump SHALL emit a JSON object containing the written or predicted version, the version it was bumped from, and the bump type:
```json
{
  "version": "<string>",
  "previous_version": "<string>",
  "bump_type": "major | minor | patch"
}
```
A successful dry-run bump SHALL emit the `--raw` schema. A successful no-op SHALL exit 0 without emitting a JSON payload. Informational messages SHALL remain off stdout when JSON output is requested.

#### Scenario: JSON output with a normal bump
- **WHEN** `grubble --output json` runs and a version bump is performed
- **THEN** stdout SHALL contain a single JSON object with the written version
- **AND** `previous_version` SHALL be the version the bump started from
- **AND** `bump_type` SHALL be the bump that was applied
- **AND** the process SHALL exit with code 0

#### Scenario: JSON output with a normal no-op
- **WHEN** `grubble --output json` runs and no version bump is needed
- **THEN** the process SHALL exit with code 0
- **AND** stdout SHALL contain no JSON payload

#### Scenario: JSON output with a dry-run no-op
- **WHEN** `grubble --dry-run --output json` runs and no version bump is needed
- **THEN** the process SHALL exit with code 0
- **AND** stdout SHALL contain no JSON payload
