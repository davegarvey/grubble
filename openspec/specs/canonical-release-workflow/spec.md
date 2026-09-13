# canonical-release-workflow Specification

## Purpose
Defines the canonical release-please-style workflow for grubble's own repo: every push to `main` triggers a dry-run version computation. If a bump is needed, a release PR is opened on a `release/v<version>` branch. A human reviews and merges the PR. The next push to `main` detects the merged release PR and creates the tag + GitHub Release on the main merge commit via the GitHub API. This matches the `semantic-release` and `googleapis/release-please` patterns.
## Requirements
### Requirement: version.yml opens a release PR on every push that warrants a bump
The workflow steps SHALL run in this order: (1) detect a merged release PR, (2) release the merged PR, (3) compute the next version with a dry run, and (4) open or update a release PR when a bump is needed. The Bump step SHALL run after the Release step so any tag just created by the Release step is visible to version analysis. The release PR's head branch SHALL be named `release/v<new_version>` and SHALL contain a single commit with the version bump and CHANGELOG entry. The Open step SHALL write the exact dry-run version with `grubble --release-version --output json`, parse the returned version, and use it for the branch and pull-request metadata. The git tag SHALL NOT be created on the release branch; the tag is created on the main merge commit after the PR is merged.

The Release step SHALL be idempotent and use the GitHub API (`gh api`). The Bump step SHALL run for every push unless `skip_version_bump` is set. The Open step SHALL be gated on the Bump step's `changed` output and its pull-request body SHALL contain the CHANGELOG entry from `grubble --changelog-entry`. This ordering handles the quiet period correctly: if v5.2.2 was released long ago and a new `fix:` lands on main, the Detect step still finds the old v5.2.2 release PR (`merged=true`), the Release step is a no-op because the tag exists, and after the Bump step computes v5.2.3, the Open step opens a fresh release PR for v5.2.3.

The version SHALL be written using `grubble --release-version`, which writes the exact dry-run version without forward-bump or sync logic. The git tag SHALL NOT be created on the release branch — the tag is created on the main merge commit after the PR is merged.

#### Scenario: a feat commit triggers a new release PR
- **WHEN** a push to `main` contains one or more `feat:` or `feat!:` commits since the last release
- **THEN** the workflow SHALL open a release PR titled `Release v<new_version>` with the version set commit on the `release/v<new_version>` branch
- **AND** the version written to Cargo.toml SHALL exactly match the dry-run prediction
- **AND** the PR body SHALL contain the full CHANGELOG entry for this version

#### Scenario: no conventional commits since the last release
- **WHEN** a push to `main` contains no `feat:`, `fix:`, or breaking-change commits since the last release
- **THEN** the workflow SHALL NOT open a release PR

#### Scenario: release PR already exists for this version
- **WHEN** a release PR for `release/v<new_version>` is already open (e.g., from a previous push)
- **THEN** the workflow SHALL update the existing PR's branch with the latest version set commit (force-push the branch)
- **AND** the PR body SHALL contain the latest CHANGELOG entry for the version

#### Scenario: step ordering prevents redundant bump after release PR merge
- **GIVEN** a release PR for v5.5.0 was just auto-merged, creating commit `C` on main
- **WHEN** the workflow runs on commit `C`
- **THEN** the Detect step finds the v5.5.0 release PR (`merged=true`)
- **AND** the Release step creates tag `v5.5.0` on commit `C` via the GitHub API
- **AND** the Bump step fetches tags, sees `v5.5.0` as the latest tag, and analyzes commits since `v5.5.0`
- **AND** the only commit since `v5.5.0` is the release commit itself (ignored by the analyser)
- **AND** the Bump step reports `changed=false`
- **AND** the Open step does NOT run (no new release PR created)

#### Scenario: new fix after a quiet period (no merged release PR for the new version)
- **GIVEN** the latest released version is v5.2.2 (with PR #79 already merged and tagged)
- **WHEN** a `fix:` commit lands on `main` that warrants v5.2.3
- **THEN** the Detect step SHALL find the old v5.2.2 release PR (`merged=true`)
- **AND** the Release step SHALL be a no-op (v5.2.2 already tagged and released)
- **AND** the Bump step SHALL report `changed=true` with `version=5.2.3`
- **AND** the Open step SHALL open a release PR titled `Release v5.2.3` on `release/v5.2.3`
- **AND** the test/build/publish jobs SHALL be skipped on this run (no new tag was created)
- **WHEN** the human merges the v5.2.3 release PR
- **THEN** the next push to `main` SHALL detect the merged v5.2.3 release PR and the Release step SHALL create the `v5.2.3` tag and GitHub Release on the merge commit

### Requirement: release PR uses an explicit merge policy
The workflow SHALL open a release PR as the review gate and SHALL use either a human merge or an explicitly configured auto-merge path. If auto-merge is enabled, the workflow SHALL use a token capable of triggering the post-merge workflow; it SHALL NOT rely on a `GITHUB_TOKEN` event that cannot trigger the next run. The release PR SHALL remain reviewable until its configured merge policy completes.

#### Scenario: workflow opens a release PR for human review
- **WHEN** auto-merge is not configured or branch protection requires human approval
- **THEN** the PR SHALL remain open and await a human merge

#### Scenario: workflow uses configured auto-merge
- **WHEN** auto-merge is configured and branch protection permits it
- **THEN** the workflow SHALL enable auto-merge with a token that can trigger the post-merge workflow

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

#### Scenario: release PR body is used as release notes
- **WHEN** the workflow creates a GitHub Release for a merged release PR
- **THEN** the release body SHALL be set to the PR body, which contains the CHANGELOG entry for that version

### Requirement: release workflow has no external AI dependency for release notes
The version workflow SHALL NOT depend on any external service (OpenAI, npm, etc.) for generating release notes. The release body SHALL be derived from the CHANGELOG.md content generated by grubble as part of the release PR. No `create-release` job or equivalent SHALL exist.

#### Scenario: release notes do not require an API key
- **WHEN** the release workflow creates a release
- **THEN** no API key SHALL be required for release notes generation

#### Scenario: build-release and publish-crate do not depend on an AI notes job
- **WHEN** the release workflow runs the build and publish jobs
- **THEN** `build-release` SHALL depend on `version` and `test` only
- **AND** `publish-crate` SHALL depend on `test` and `build-release` only

### Requirement: v<major> floating tag follows the latest release
The workflow SHALL keep the `v<major>` floating tag in sync with the latest release. After creating a `v<version>` tag for a release, the workflow SHALL update `v<major>` to point to the same commit.

#### Scenario: minor release updates v<major>
- **WHEN** a release for `5.2.2` is tagged on commit `C`
- **THEN** the `v5` tag SHALL be updated to point to commit `C`

### Requirement: Bump step uses dry-run to compute the next version
The Bump step in the workflow SHALL use `grubble --dry-run` to compute the next version without writing files. The Bump step SHALL then determine whether a release is needed by comparing the dry-run output version to the current Cargo.toml version. If they differ, a release is needed; otherwise it is not.

The Bump step's output version SHALL be used both to determine `changed=true/false` AND as the argument to `grubble --release-version` in the Open step. Because `--release-version` writes the exact version without sync logic, the dry-run version and the written version SHALL always match.

#### Scenario: dry-run returns the same version
- **WHEN** `grubble --dry-run` returns the same version as Cargo.toml
- **THEN** the Bump step SHALL report `changed=false`; no release PR SHALL be opened

#### Scenario: dry-run returns a new version
- **WHEN** `grubble --dry-run` returns a version that differs from Cargo.toml (e.g., 5.2.0 → 5.2.1)
- **THEN** the Bump step SHALL report `changed=true`
- **AND** the dry-run version SHALL be used as the `--release-version` argument in the Open step
- **AND** the version written to Cargo.toml SHALL be identical to the dry-run version

### Requirement: Bump step fetches tags before dry-run
The Bump step SHALL run `git fetch origin --tags --force` before invoking grubble's dry-run. This ensures tags created by the preceding Release step (via the GitHub API) are visible to the local repository, preventing the dry-run from using a stale last-tag. This fetch is required even though the checkout step uses `fetch-tags: true` because the Release step creates tags via the GitHub API, not through git.

#### Scenario: Release step creates a new tag moments before Bump step
- **GIVEN** the Release step just created tag `v5.3.2` on the merge commit
- **WHEN** the Bump step runs `grubble --dry-run --raw`
- **THEN** `git fetch origin --tags --force` SHALL have been called before grubble
- **AND** grubble SHALL find `v5.3.2` as the latest tag via `git describe --tags`
- **AND** the dry-run output SHALL reflect the correct next version after `v5.3.2`

The Open step SHALL also run `git fetch origin --tags --force` before invoking `grubble --release-version --changelog` (defence in depth — the dry-run already fetched, but the CHANGELOG generation also depends on `get_last_tag()`).

### Requirement: CHANGELOG.md is updated as part of the release commit
The release commit on the `release/v<version>` branch SHALL include an update to `CHANGELOG.md` with the new release entry, generated by `grubble --release-version --changelog`. The release PR's diff SHALL show both the Cargo.toml version set and the CHANGELOG.md addition.

#### Scenario: release commit contains CHANGELOG entry
- **WHEN** a release PR is opened for version 5.2.1
- **THEN** the PR diff SHALL include a new `## [5.2.1] - <date>` entry in `CHANGELOG.md`

### Requirement: workflow cleans up unreachable tags before bump
The "Clean up stale tags" step SHALL run before any tag-dependent analysis. It SHALL remove any local `v*` tag that is not reachable from `main` (i.e., the tag points to a commit that is not an ancestor of `main`). This prevents old failed runs from leaving orphan tags that interfere with version detection. It runs before the Release step so that stale tags do not interfere with the dry-run's `git describe --tags` computation.

#### Scenario: orphan tag from a previous failed run
- **WHEN** a previous failed run left a `v5.2.0` tag on a release branch commit that was never merged
- **THEN** the stale tag cleanup SHALL remove the local `v5.2.0` tag so the next bump step computes the correct next version
