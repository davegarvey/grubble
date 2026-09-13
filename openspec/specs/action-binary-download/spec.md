# action-binary-download Specification

## Purpose

Defines how the composite GitHub Action selects a released grubble binary from the action reference and verifies that the downloaded artifacts belong to the same release.

## Requirements

### Requirement: Version-aware binary download
The action SHALL determine which version of the grubble binary to download based on the user's pin, rather than always downloading the latest release.

#### Scenario: User pins a specific semver tag
- **WHEN** a user pins `uses: davegarvey/grubble@v5.2.1`
- **THEN** the action SHALL download the binary from the `v5.2.1` release

#### Scenario: User pins a floating major version tag
- **WHEN** a user pins `uses: davegarvey/grubble@v5`
- **THEN** the action SHALL fall back to the latest release and SHALL emit a warning

#### Scenario: User pins a branch ref
- **WHEN** a user pins `uses: davegarvey/grubble@main`
- **THEN** the action SHALL fall back to the latest release and SHALL emit a warning

### Requirement: Version resolution from github.action_ref
The action SHALL use `github.action_ref` as the primary version source and normalize it before use.

#### Scenario: Ref is a specific semver tag
- **WHEN** `github.action_ref` is `v5.2.1`
- **THEN** the action SHALL query `repos/davegarvey/grubble/releases/tags/v5.2.1` to validate the release exists

#### Scenario: Ref contains a ref prefix
- **WHEN** `github.action_ref` is `refs/tags/v5.2.1` or `refs/heads/main`
- **THEN** the action SHALL strip the `refs/tags/` or `refs/heads/` prefix before processing

#### Scenario: Ref does not match a semver pattern
- **WHEN** `github.action_ref` is `v5`, `main`, or a SHA
- **THEN** the action SHALL skip the tag-specific API call and fall back to `releases/latest`

#### Scenario: Specific release tag cannot be resolved
- **WHEN** `github.action_ref` matches a semver tag but that release cannot be found
- **THEN** the action SHALL warn and fall back to the latest available release

### Requirement: Fallback with warning
When the action falls back to the latest release, it SHALL emit a visible warning so users understand the download may not match their pin.

#### Scenario: Fallback to latest release
- **WHEN** the action cannot resolve a specific release tag from `github.action_ref`
- **THEN** the action SHALL emit `::warning::` with a message indicating the fallback and the ref value

### Requirement: Checksum download matches binary download
The checksum file SHALL be downloaded from the same resolved version as the binary.

#### Scenario: Checksum uses resolved version
- **WHEN** the binary is downloaded from version `v5.2.1`
- **THEN** the checksum SHALL also be downloaded from `v5.2.1`

### Requirement: Resolved release must provide downloadable assets
The action SHALL verify that the resolved release provides a downloadable binary asset before attempting the download. If no binary assets are available, the action SHALL fail with an actionable error naming the release and explaining how to choose an available version.

#### Scenario: Resolved release has no assets
- **WHEN** a pinned release exists but has no downloadable binary assets
- **THEN** the action SHALL emit an error naming the release
- **AND** the action SHALL fail before attempting the binary download
