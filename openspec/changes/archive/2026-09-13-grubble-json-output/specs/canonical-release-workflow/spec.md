## MODIFIED Requirements

### Requirement: Open step derives branch name from actual version written

The Open step SHALL run grubble on a temporary branch to perform the version bump and CHANGELOG update. After grubble completes, the step SHALL read the actual new version from Cargo.toml and derive the release branch name as `release/v<actual_version>`. The branch SHALL be renamed and pushed under this derived name.

The version SHALL be obtained from grubble's `--output json` output, parsed with `jq -r '.version'`. This replaces the previous approach of grepping Cargo.toml directly.

#### Scenario: sync logic fires during Open step
- **GIVEN** Cargo.toml is version `5.3.1` and latest tag is `v5.4.0` (Cargo.toml is behind the tag)
- **WHEN** the Open step runs grubble with `--output json`
- **THEN** grubble SHALL sync Cargo.toml to `5.4.0` first, then compute and write `5.4.1` (patch bump from synced version)
- **AND** the JSON output SHALL contain `{"version": "5.4.1"}`
- **AND** the Open step SHALL parse the version from the JSON output
- **AND** the release branch SHALL be named `release/v5.4.1`

#### Scenario: grubble writes the same version as the dry-run predicted
- **GIVEN** Cargo.toml is `5.3.0`, latest tag is `v5.3.0`, and dry-run predicts `5.3.1`
- **WHEN** the Open step runs grubble with `--output json`
- **THEN** grubble SHALL write `5.3.1` to Cargo.toml (no sync needed)
- **AND** the JSON output SHALL contain `{"version": "5.3.1"}`
- **AND** the release branch SHALL be named `release/v5.3.1`
