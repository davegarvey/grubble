## ADDED Requirements

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
