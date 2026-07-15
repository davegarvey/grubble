# canonical-release-workflow (delta)

## MODIFIED Requirements

### Requirement: version.yml opens a release PR on every push that warrants a bump
The `version.yml` workflow SHALL run on every push to `main`. When the Bump step detects a conventional commit since the last release that warrants a version bump, the workflow SHALL open (or update) a release PR. The release PR's head branch SHALL be named `release/v<new_version>` and SHALL contain a single commit with the version bump (Cargo.toml, CHANGELOG.md). The new version SHALL be written to Cargo.toml and CHANGELOG.md using `grubble --release-version`, which writes the exact version without forward-bump or sync logic. The git tag SHALL NOT be created on the release branch — the tag is created on the main merge commit after the PR is merged (see "workflow tags the merge commit" requirement below).

The Open step is independent of the Release step: it SHALL run whenever a new release is needed (Bump step reports `changed=true`), regardless of whether a previously-merged release PR is still being processed by the Release step. The two steps do not race — they run sequentially within a single job, and the Release step uses the GitHub API (`gh api`) which is independent of the Open step's branch creation and `gh pr create` call. This decoupling is what makes the workflow correct after a "quiet period": if v5.2.2 was released long ago and a new `fix:` lands on main, the Bump step computes v5.2.3 (`changed=true`), the Detect step still finds the old v5.2.2 release PR (`merged=true`), and the Open step opens a fresh release PR for v5.2.3.

#### Scenario: a feat commit triggers a new release PR
- **WHEN** a push to `main` contains one or more `feat:` or `feat!:` commits since the last release
- **THEN** the workflow SHALL open a release PR titled `Release v<new_version>` with a single version-set commit on `release/v<new_version>`
- **AND** the version written to Cargo.toml SHALL exactly match the dry-run prediction

#### Scenario: no conventional commits since the last release
- **WHEN** a push to `main` contains no `feat:`, `fix:`, or breaking-change commits since the last release
- **THEN** the workflow SHALL NOT open a release PR

#### Scenario: release PR already exists for this version
- **WHEN** a release PR for `release/v<new_version>` is already open (e.g., from a previous push)
- **THEN** the workflow SHALL update the existing PR's branch with the latest version-set commit (force-push the branch)

#### Scenario: new fix after a quiet period (no merged release PR for the new version)
- **GIVEN** the latest released version is v5.2.2 (with PR #79 already merged and tagged)
- **WHEN** a `fix:` commit lands on `main` that warrants v5.2.3
- **THEN** the Bump step SHALL report `changed=true` with `version=5.2.3`, the Open step SHALL open a release PR titled `Release v5.2.3` on `release/v5.2.3`, and the Release step SHALL run but be a no-op (v5.2.2 already tagged and released)
- **AND** the test/build/publish jobs SHALL be skipped on this run (the new release PR has not been merged yet)
- **WHEN** the human merges the v5.2.3 release PR
- **THEN** the next push to `main` SHALL detect the merged v5.2.3 release PR and the Release step SHALL create the `v5.2.3` tag and GitHub Release on the merge commit

### MODIFIED: Requirement: Bump step uses dry-run to compute the next version
The Bump step in the workflow SHALL use `grubble --dry-run` to compute the next version without writing files. The Bump step SHALL then determine whether a release is needed by comparing the dry-run output version to the current Cargo.toml version. If they differ, a release is needed; otherwise it is not.

The Bump step's output version SHALL be used both to determine `changed=true/false` AND to determine the release version passed to `grubble --release-version` in the Open step. Because `--release-version` writes the exact version without sync logic, the dry-run version and the actual written version SHALL always match.

#### Scenario: dry-run returns the same version
- **WHEN** `grubble --dry-run` returns the same version as Cargo.toml
- **THEN** the Bump step SHALL report `changed=false`; no release PR SHALL be opened

#### Scenario: dry-run returns a new version
- **WHEN** `grubble --dry-run` returns a version that differs from Cargo.toml (e.g., 5.2.0 → 5.2.1)
- **THEN** the Bump step SHALL report `changed=true`
- **AND** the dry-run version SHALL be used as the `--release-version` argument in the Open step
- **AND** the version written to Cargo.toml SHALL be identical to the dry-run version

### REMOVED: Requirement: Bump step dry-run version differs from actual version due to sync
**Reason**: Replaced by `--release-version` which writes the exact dry-run version. Sync logic no longer runs during the Open step, so the dry-run and actual versions cannot diverge.
**Migration**: The `--release-version` flag enforces that the written version matches the dry-run prediction.

### REMOVED: Scenario: sync logic fires during Open step
**Reason**: The Open step no longer runs forward-bump logic. It uses `--release-version` which has no sync logic.
**Migration**: Use `grubble --release-version <VERSION>` instead of forward-bump in the Open step.

### REMOVED: Requirement: Open step derives branch name from actual version written
**Reason**: With `--release-version`, the dry-run version and actual version are always identical. The branch name can be derived directly from the dry-run output.
**Migration**: The branch name is derived from `steps.bump.outputs.version` which equals the written version.

### REMOVED: Requirement: Open step cleans up stale branches and PRs when version diverges
**Reason**: Version divergence no longer occurs because `--release-version` writes the exact dry-run version.
**Migration**: No stale branch cleanup is needed.

### REMOVED: Requirement: Open step uses actual version for PR title and body
**Reason**: The dry-run version and actual version are always identical. The PR title uses the dry-run version directly.
**Migration**: Use `steps.bump.outputs.version` for PR title and body.

### MODIFIED: Requirement: Open step fetches tags before running grubble
The Open step SHALL run `git fetch origin --tags --force` before invoking grubble. This ensures tags created by the preceding Release step are visible to the local repository, preventing grubble's `get_last_tag()` from returning a stale tag when generating the CHANGELOG entry via `--release-version --changelog`.

#### Scenario: Release step creates a new tag moments before Open step
- **GIVEN** the Release step just created tag `v5.3.2` on the merge commit
- **WHEN** the Open step runs `grubble --release-version 5.3.3 --changelog`
- **THEN** `git fetch origin --tags --force` SHALL have been called before grubble
- **AND** grubble SHALL find `v5.3.2` as the latest tag via `git describe --tags`
- **AND** the CHANGELOG entry SHALL include commits since `v5.3.2`

### MODIFIED: Requirement: CHANGELOG.md is updated as part of the release commit
The release commit on the `release/v<version>` branch SHALL include an update to `CHANGELOG.md` with the new release entry, generated by `grubble --release-version --changelog`. The release PR's diff SHALL show both the Cargo.toml version update and the CHANGELOG.md addition.

#### Scenario: release commit contains CHANGELOG entry
- **WHEN** a release PR is opened for version 5.2.1
- **THEN** the PR diff SHALL include a new `## [5.2.1] - <date>` entry in `CHANGELOG.md`
