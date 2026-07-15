## MODIFIED Requirements

### Requirement: Bump step uses dry-run to compute the next version

The Bump step in the workflow SHALL use `grubble --dry-run` to compute the next version without writing files. The Bump step SHALL then determine whether a release is needed by comparing the dry-run output version to the current Cargo.toml version. If they differ, a release is needed; otherwise it is not.

The Bump step's output version SHALL be used only to determine `changed=true/false`. It SHALL NOT be used to determine the release branch name. The release branch name SHALL be derived from the actual version written to Cargo.toml after grubble completes in the Open step.

#### Scenario: dry-run returns the same version
- **WHEN** `grubble --dry-run` returns the same version as Cargo.toml
- **THEN** the Bump step SHALL report `changed=false`; no release PR SHALL be opened

#### Scenario: dry-run returns a new version
- **WHEN** `grubble --dry-run` returns a version that differs from Cargo.toml (e.g., 5.2.0 → 5.2.1)
- **THEN** the Bump step SHALL report `changed=true`; the workflow SHALL proceed to open or update a release PR

#### Scenario: dry-run version differs from actual version due to sync
- **GIVEN** Cargo.toml is `5.3.1` and the latest tag is `v5.4.0` (file is behind tag)
- **WHEN** the dry-run outputs `5.3.2` (computed from the stale file version, skipping sync)
- **THEN** the `changed` boolean SHALL be `true` (5.3.2 != 5.3.1)
- **AND** the dry-run's `version` output (`5.3.2`) SHALL NOT be used for branch naming, only for the `changed` signal
- **AND** the actual version written by grubble in the Open step SHALL be `5.4.1` (sync to 5.4.0 + patch bump)
- **AND** the release branch SHALL be named `release/v5.4.1`

### Requirement: Open step derives branch name from actual version written

The Open step SHALL run grubble on a temporary branch to perform the version bump and CHANGELOG update. After grubble completes, the step SHALL read the actual new version from Cargo.toml and derive the release branch name as `release/v<actual_version>`. The branch SHALL be renamed and pushed under this derived name.

This ensures the release branch name always matches the version grubble actually wrote, accounting for any sync logic that may have triggered during the run.

#### Scenario: sync logic fires during Open step
- **GIVEN** Cargo.toml is version `5.3.1` and latest tag is `v5.4.0` (Cargo.toml is behind the tag)
- **WHEN** the Open step runs grubble
- **THEN** grubble SHALL sync Cargo.toml to `5.4.0` first, then compute and write `5.4.1` (patch bump from synced version)
- **AND** the Open step SHALL read `5.4.1` from Cargo.toml after grubble completes
- **AND** the release branch SHALL be named `release/v5.4.1`

#### Scenario: grubble writes the same version as the dry-run predicted
- **GIVEN** Cargo.toml is `5.3.0`, latest tag is `v5.3.0`, and dry-run predicts `5.3.1`
- **WHEN** the Open step runs grubble
- **THEN** grubble SHALL write `5.3.1` to Cargo.toml (no sync needed)
- **AND** the Open step SHALL read `5.3.1` from Cargo.toml after grubble completes
- **AND** the release branch SHALL be named `release/v5.3.1`

### Requirement: Open step cleans up stale branches and PRs when version diverges

When the sync logic causes the actual version written by grubble to differ from the dry-run version, any existing release branch and PR for the dry-run version SHALL be closed and deleted. This prevents orphaned branches and PRs from confusing human reviewers.

#### Scenario: stale release PR from previous run is closed when version changes
- **GIVEN** a previous workflow run opened PR #118 on branch `release/v5.3.2` (from dry-run)
- **AND** the current run's dry-run predicts `v5.3.2` again
- **AND** the current run's sync logic causes grubble to write version `5.4.1`
- **WHEN** the Open step detects the version divergence
- **THEN** the Open step SHALL close PR #118 with a comment "Superseded by release/v5.4.1 — version changed due to drift repair"
- **AND** the Open step SHALL delete the `release/v5.3.2` remote branch
- **AND** the Open step SHALL create a new PR on `release/v5.4.1`

### Requirement: Open step uses actual version for PR title and body

The release PR title and body SHALL use the actual version read from Cargo.toml after grubble completes, not the dry-run version.

#### Scenario: PR title reflects actual version
- **GIVEN** dry-run predicts `5.3.2` but grubble writes `5.4.1`
- **WHEN** the Open step creates a release PR
- **THEN** the PR title SHALL be `Release v5.4.1`
- **AND** the PR body SHALL be the CHANGELOG entry for `5.4.1`

### Requirement: Open step fetches tags before running grubble

The Open step SHALL run `git fetch origin --tags --force` before invoking grubble. This ensures tags created by the preceding Release step are visible to the local repository, preventing grubble's `get_last_tag()` from returning a stale tag.

#### Scenario: Release step creates a new tag moments before Open step
- **GIVEN** the Release step just created tag `v5.3.2` on the merge commit
- **WHEN** the Open step runs grubble
- **THEN** `git fetch origin --tags --force` SHALL have been called before grubble
- **AND** grubble SHALL find `v5.3.2` as the latest tag via `git describe --tags`

### Requirement: Temporary branch naming prevents conflicts

The temporary branch used before the version is known SHALL be named `release/_tmp` or use a naming scheme that avoids collisions. The `concurrency` group on the workflow SHALL prevent concurrent runs, so no collision between workflow runs is possible.

#### Scenario: concurrent workflow runs
- **WHEN** two workflow runs attempt to create a release PR simultaneously
- **THEN** the second run SHALL wait for the first to complete (due to `concurrency` group)
- **AND** no temporary branch collision SHALL occur
