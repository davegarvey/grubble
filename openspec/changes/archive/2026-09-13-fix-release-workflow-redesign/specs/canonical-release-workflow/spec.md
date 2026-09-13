## MODIFIED Requirements

### Requirement: version.yml opens a release PR on every push that warrants a bump
The `version.yml` workflow SHALL run on every push to `main`. The workflow steps SHALL run in this order: (1) detect a merged release PR, (2) release the merged PR, (3) compute the next version with a dry run, and (4) open or update a release PR when a bump is needed. The release PR's head branch SHALL be named `release/v<new_version>` and SHALL contain a single commit with the version bump. The Open step SHALL use `grubble --release-version` to write the exact version reported by the Bump step, without forward-bump or file/tag synchronization logic, and SHALL parse its `--output json` result rather than scraping the package file. The git tag SHALL NOT be created on the release branch; the tag is created on the main merge commit after the PR is merged.

The Bump step SHALL run before the Open step and the two steps SHALL execute sequentially after the Release step. This makes the dry-run result authoritative for the release branch, package-file version, and pull-request title.

#### Scenario: a feat commit triggers a new release PR
- **WHEN** a push to `main` contains one or more `feat:` or `feat!:` commits since the last release
- **THEN** the workflow SHALL open a release PR titled `Release v<new_version>` with the version set commit on the `release/v<new_version>` branch
- **AND** the version written to Cargo.toml SHALL exactly match the dry-run prediction
- **AND** the Open step SHALL obtain the written version from the JSON output of `grubble`

#### Scenario: no conventional commits since the last release
- **WHEN** a push to `main` contains no `feat:`, `fix:`, or breaking-change commits since the last release
- **THEN** the workflow SHALL NOT open a release PR

#### Scenario: release PR already exists for this version
- **WHEN** a release PR for `release/v<new_version>` is already open (e.g., from a previous push)
- **THEN** the workflow SHALL update the existing PR's branch with the latest version set commit (force-push the branch)

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

### Requirement: Bump step uses dry-run to compute the next version
The Bump step in the workflow SHALL use `grubble --dry-run` to compute the next version without writing files. The Bump step SHALL then determine whether a release is needed by comparing the dry-run output version to the current Cargo.toml version. If they differ, a release is needed; otherwise it is not.

The Bump step's output version SHALL be used both to determine `changed=true/false` AND as the argument to `grubble --release-version` in the Open step. Because `--release-version` writes the exact version without sync logic, the dry-run version and the written version SHALL always match.

#### Scenario: dry-run returns the same version
- **WHEN** `grubble --dry-run` returns the same version as Cargo.toml
- **THEN** the Bump step SHALL report `changed=false`; no release PR SHALL be opened

#### Scenario: dry-run returns a new version
- **WHEN** `grubble --dry-run` returns a version that differs from Cargo.toml (e.g., 5.2.0 -> 5.2.1)
- **THEN** the Bump step SHALL report `changed=true`
- **AND** the dry-run version SHALL be used as the `--release-version` argument in the Open step
- **AND** the version written to Cargo.toml SHALL be identical to the dry-run version

### Requirement: CHANGELOG.md is updated as part of the release commit
The release commit on the `release/v<version>` branch SHALL include an update to `CHANGELOG.md` with the new release entry, generated by `grubble --release-version --changelog`. The release PR's diff SHALL show both the Cargo.toml version update and the CHANGELOG.md addition.

#### Scenario: release commit contains CHANGELOG entry
- **WHEN** a release PR is opened for version 5.2.1
- **THEN** the PR diff SHALL include a new `## [5.2.1] - <date>` entry in `CHANGELOG.md`
