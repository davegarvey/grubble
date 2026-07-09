## ADDED Requirements

### Requirement: `--output` flag controls output format
The `grubble` CLI SHALL accept an `--output` flag with values `text` (default) and `json`. The flag applies only to `--bump-type` and `--raw` modes. When `--output` is set to any value other than `text` or `json`, the program exits with a non-zero code and a clear error.

#### Scenario: Default text output
- **WHEN** `grubble --bump-type` runs without `--output`
- **THEN** the output format is `text`
- **AND** the program prints `major`, `minor`, `patch`, or `none` to stdout

#### Scenario: Explicit text output
- **WHEN** `grubble --bump-type --output text` runs
- **THEN** the program prints `major`, `minor`, `patch`, or `none` to stdout

#### Scenario: Invalid output value
- **WHEN** `grubble --bump-type --output yaml` runs
- **THEN** the program exits with a non-zero code
- **AND** a clear error message naming the valid values is written to stderr

### Requirement: `--bump-type --output json` schema
The `grubble --bump-type --output json` mode SHALL write a single JSON object to stdout with the following structure:
```json
{
  "bump_type": "major | minor | patch | none",
  "current_version": "<string>",
  "triggering_commits": ["<string>"],
  "unknown_commits": ["<string>"]
}
```

The JSON object MUST be the only content written to stdout in this mode (no informational logs, no progress text).

#### Scenario: JSON output for a minor bump
- **WHEN** `grubble --bump-type --output json` runs against a repo with a `feat:` commit
- **THEN** stdout contains a single JSON object
- **AND** `bump_type` is `"minor"`
- **AND** `current_version` is the parsed current version string
- **AND** `triggering_commits` is a non-empty array containing the triggering commit
- **AND** `unknown_commits` is an empty array (or absent)

#### Scenario: JSON output for no bump
- **WHEN** `grubble --bump-type --output json` runs with no triggering commits
- **THEN** stdout contains a single JSON object
- **AND** `bump_type` is `"none"`
- **AND** `current_version` is the parsed current version string
- **AND** `triggering_commits` is an empty array
- **AND** `unknown_commits` contains any unrecognized commit messages

#### Scenario: JSON output for a major bump
- **WHEN** `grubble --bump-type --output json` runs against a repo with a `feat!:` commit
- **THEN** stdout contains a single JSON object
- **AND** `bump_type` is `"major"`

### Requirement: `--raw --output json` schema
The `grubble --raw --output json` mode SHALL write a single JSON object to stdout with the following structure:
```json
{
  "version": "<string>",
  "preset": "<string>"
}
```

The JSON object MUST be the only content written to stdout in this mode.

#### Scenario: JSON raw output
- **WHEN** `grubble --raw --output json --preset rust` runs in a directory with `Cargo.toml` containing `version = "1.2.3"`
- **THEN** stdout contains a single JSON object
- **AND** `version` is `"1.2.3"`
- **AND** `preset` is `"rust"`

#### Scenario: JSON raw output with default preset
- **WHEN** `grubble --raw --output json` runs in a git repo with a `v1.2.3` tag and no `--preset`
- **THEN** stdout contains a single JSON object
- **AND** `version` is `"1.2.3"`
- **AND** `preset` is `"git"`

### Requirement: JSON output is invalid for non-applicable modes
The `grubble` CLI SHALL exit with a non-zero code when `--output` is used with the normal run mode or `--dry-run`.

#### Scenario: JSON output with normal run
- **WHEN** `grubble --output json` runs (no `--bump-type` or `--raw`)
- **THEN** the program exits with a non-zero code
- **AND** a clear error message is written to stderr

#### Scenario: JSON output with dry-run
- **WHEN** `grubble --dry-run --output json` runs
- **THEN** the program exits with a non-zero code
- **AND** a clear error message is written to stderr
