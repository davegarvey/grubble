# canonical-release-workflow Specification

## Purpose
Defines the canonical release-please-style workflow for grubble's own repo: every push to `main` triggers a dry-run version computation. If a bump is needed, a release PR is opened on a `release/v<version>` branch. A human reviews and merges the PR. The next push to `main` detects the merged release PR and creates the tag + GitHub Release on the main merge commit via the GitHub API. This matches the `semantic-release` and `googleapis/release-please` patterns.
## Requirements
### Requirement: version.yml opens a release PR on every push that warrants a bump
The `version.yml` workflow SHALL run on every push to `main`. When the Bump step detects a conventional commit since the last release that warrants a version bump, the workflow SHALL open (or update) a release PR. The release PR's head branch SHALL be named `release/v<new_version>` and SHALL contain a single commit with the version bump (Cargo.toml, CHANGELOG.md). The git tag SHALL NOT be created on the release branch — the tag is created on the main merge commit after the PR is merged (see "workflow tags the merge commit" requirement below).

The Open step is independent of the Release step: it SHALL run whenever a new release is needed (Bump step reports `changed=true`), regardless of whether a previously-merged release PR is still being processed by the Release step. The two steps do not race -- they run sequentially within a single job, and the Release step uses the GitHub API (`gh api`) which is independent of the Open step's branch creation and `gh pr create` call. This decoupling is what makes the workflow correct after a "quiet period": if v5.2.2 was released long ago and a new `fix:` lands on main, the Bump step computes v5.2.3 (`changed=true`), the Detect step still finds the old v5.2.2 release PR (`merged=true`), and the Open step opens a fresh release PR for v5.2.3.

#### Scenario: a feat commit triggers a new release PR
- **WHEN** a push to `main` contains one or more `feat:` or `feat!:` commits since the last release
- **THEN** the workflow SHALL open a release PR titled `Release v<new_version>` with the version bump commit on the `release/v<new_version>` branch

#### Scenario: no conventional commits since the last release
- **WHEN** a push to `main` contains no `feat:`, `fix:`, or breaking-change commits since the last release
- **THEN** the workflow SHALL NOT open a release PR

#### Scenario: release PR already exists for this version
- **WHEN** a release PR for `release/v<new_version>` is already open (e.g., from a previous push)
- **THEN** the workflow SHALL update the existing PR's branch with the latest version bump commit (force-push the branch)

#### Scenario: new fix after a quiet period (no merged release PR for the new version)
- **GIVEN** the latest released version is v5.2.2 (with PR #79 already merged and tagged)
- **WHEN** a `fix:` commit lands on `main` that warrants v5.2.3
- **THEN** the Bump step SHALL report `changed=true` with `version=5.2.3`, the Open step SHALL open a release PR titled `Release v5.2.3` on `release/v5.2.3`, and the Release step SHALL run but be a no-op (v5.2.2 already tagged and released)
- **AND** the test/build/publish jobs SHALL be skipped on this run (the new release PR has not been merged yet)
- **WHEN** the human merges the v5.2.3 release PR
- **THEN** the next push to `main` SHALL detect the merged v5.2.3 release PR and the Release step SHALL create the `v5.2.3` tag and GitHub Release on the merge commit

### Requirement: release PR is not auto-merged
The workflow SHALL NOT call `gh pr merge --auto` or otherwise enable auto-merge on the release PR. The release PR SHALL remain open until a human merges it. This matches the canonical release-please pattern and is the security model: a release is a high-impact event and requires human review.

#### Scenario: workflow opens release PR without auto-merge
- **WHEN** the workflow opens a release PR
- **THEN** the PR's `auto_merge` flag SHALL remain unset; the PR is open and awaits human review

### Requirement: workflow tags the merge commit on the next run after a release PR is merged
On every push to `main`, the workflow SHALL detect the most recent merged release PR whose head branch matches `^release/v\d+\.\d+\.\d+$`. When such a merged PR is found, the workflow SHALL:
1. Create a git tag `v<new_version>` pointing to the merge commit SHA via the GitHub API
2. Create a GitHub Release for the tag with the CHANGELOG entry as the release body
3. Update the `v<major>` floating tag to point to the same merge commit SHA
4. Emit step outputs that drive the downstream test/build/publish jobs

#### Scenario: release PR is squash-merged
- **WHEN** the most recent merged release PR was merged with squash
- **THEN** the `v<version>` tag SHALL point to the squash commit SHA; the GitHub Release is created on the same SHA

#### Scenario: release PR is merge-merged
- **WHEN** the most recent merged release PR was merged with a merge commit
- **THEN** the `v<version>` tag SHALL point to the merge commit SHA; the GitHub Release is created on the same SHA

#### Scenario: no merged release PR exists
- **WHEN** no release PR has been merged since the last successful release
- **THEN** the workflow SHALL NOT create a tag or release; the downstream jobs SHALL NOT be triggered

#### Scenario: tag creation is idempotent
- **WHEN** the `v<version>` tag already exists on the correct commit
- **THEN** the workflow SHALL detect this and SHALL NOT fail or re-create the tag

### Requirement: v<major> floating tag follows the latest release
The workflow SHALL keep the `v<major>` floating tag in sync with the latest release. After creating a `v<version>` tag for a release, the workflow SHALL update `v<major>` to point to the same commit.

#### Scenario: minor release updates v<major>
- **WHEN** a release for `5.2.2` is tagged on commit `C`
- **THEN** the `v5` tag SHALL be updated to point to commit `C`

### Requirement: Bump step uses dry-run to compute the next version
The Bump step in the workflow SHALL use `grubble --dry-run` to compute the next version without writing files. The Bump step SHALL then determine whether a release is needed by comparing the dry-run output version to the current Cargo.toml version. If they differ, a release is needed; otherwise it is not.

The Bump step's output version SHALL be used only to determine `changed=true/false`. It SHALL NOT be used to determine the release branch name. The release branch name SHALL be derived from the actual version written to Cargo.toml after grubble completes in the Open step. This is because `--dry-run` forces `config.raw = true` internally, which skips grubble's file-behind-tag sync logic; the dry-run version can differ from what grubble actually writes when sync fires.

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

### Requirement: CHANGELOG.md is updated as part of the release commit
The release commit on the `release/v<version>` branch SHALL include an update to `CHANGELOG.md` with the new release entry, generated by `grubble --changelog`. The release PR's diff SHALL show both the Cargo.toml version bump and the CHANGELOG.md addition.

#### Scenario: release commit contains CHANGELOG entry
- **WHEN** a release PR is opened for version 5.2.1
- **THEN** the PR diff SHALL include a new `## [5.2.1] - <date>` entry in `CHANGELOG.md`

### Requirement: workflow cleans up unreachable tags before the next bump
The "Clean up stale tags" step SHALL run before the Bump step. It SHALL remove any local `v*` tag that is not reachable from `main` (i.e., the tag points to a commit that is not an ancestor of `main`). This prevents old failed runs from leaving orphan tags that interfere with version detection.

#### Scenario: orphan tag from a previous failed run
- **WHEN** a previous failed run left a `v5.2.0` tag on a release branch commit that was never merged
- **THEN** the stale tag cleanup SHALL remove the local `v5.2.0` tag so the next bump step computes the correct next version
