## ADDED Requirements

### Requirement: Get current version via node preset
The "Get current version" step SHALL use `node -p "require('./package.json').version"` when `preset: node` is configured, instead of `./grubble --raw`. If the command fails (e.g., `package.json` not found), the version SHALL fall back to `0.0.0`.

#### Scenario: node preset reads from package.json
- **WHEN** `preset: node` is configured and `./package.json` exists with `"version": "0.1.0"`
- **THEN** the detected current version SHALL be `0.1.0`

#### Scenario: node preset fallback on missing file
- **WHEN** `preset: node` is configured but `./package.json` does not exist
- **THEN** the detected current version SHALL be `0.0.0`

#### Scenario: node preset fallback on invalid version
- **WHEN** `preset: node` is configured and `./package.json` exists but has no `version` field
- **THEN** the detected current version SHALL be `0.0.0`

### Requirement: Get current version via rust preset
The "Get current version" step SHALL use `grep '^version' Cargo.toml | head -1 | cut -d'"' -f2` when `preset: rust` is configured, instead of `./grubble --raw`. If the command fails (e.g., `Cargo.toml` not found or no version field), the version SHALL fall back to `0.0.0`.

#### Scenario: rust preset reads from Cargo.toml
- **WHEN** `preset: rust` is configured and `./Cargo.toml` contains `version = "1.2.3"`
- **THEN** the detected current version SHALL be `1.2.3`

#### Scenario: rust preset fallback on missing file
- **WHEN** `preset: rust` is configured but `./Cargo.toml` does not exist
- **THEN** the detected current version SHALL be `0.0.0`

### Requirement: Get current version via git preset or no preset
The "Get current version" step SHALL continue to use `./grubble --raw` when `preset: git` is configured, or when no preset is configured. This preserves the existing behavior where `--raw` uses GitStrategy (reads from git tags).

#### Scenario: git preset with existing tags
- **WHEN** `preset: git` is configured and git tag `v2.0.0` exists
- **THEN** `./grubble --raw` SHALL be used and return `2.0.0`

#### Scenario: git preset with no tags
- **WHEN** `preset: git` is configured and no git tags exist
- **THEN** `./grubble --raw` SHALL be used and return `0.0.0` (GitStrategy default)

### Requirement: previous_version set in bump step
The "Run bump" step SHALL write `previous_version=...` to its `$GITHUB_OUTPUT`, so the composite action's declared `previous-version` output resolves to a value rather than being empty/null.

#### Scenario: previous_version written alongside version
- **WHEN** the "Run bump" step executes after a successful version bump
- **THEN** `$GITHUB_OUTPUT` SHALL contain a line `previous_version=<the pre-bump version>`

#### Scenario: previous_version equals the pre-bump version
- **WHEN** the pre-bump version was `0.1.0` and the new version is `1.0.0`
- **THEN** the `previous-version` output of the action SHALL be `0.1.0`
