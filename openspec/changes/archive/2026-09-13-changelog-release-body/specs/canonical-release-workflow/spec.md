## MODIFIED Requirements

### Requirement: version.yml opens a release PR on every push that warrants a bump
The `version.yml` workflow SHALL run on every push to `main`. The workflow steps SHALL run in this order: (1) detect a merged release PR, (2) release the merged PR, (3) compute the next version with a dry run, and (4) open or update a release PR when a bump is needed. The release PR's head branch SHALL be named `release/v<new_version>` and SHALL contain a single commit with the version bump and CHANGELOG entry. The Open step SHALL write the exact dry-run version with `grubble --release-version --output json` and SHALL use the parsed version for the branch and pull-request metadata. The git tag SHALL NOT be created on the release branch; the tag is created on the main merge commit after the PR is merged.

The release PR body SHALL contain the CHANGELOG entry for this version, obtained from `grubble --changelog-entry`, rather than a placeholder or externally generated release note. The body SHALL be used as the GitHub Release notes after the PR is merged.

The Bump step SHALL run after the Release step so a tag created by the Release step is visible to version analysis. The Open step SHALL run whenever a new release is needed, including when a previously merged release PR is being processed in the same workflow run.

#### Scenario: a feat commit triggers a new release PR
- **WHEN** a push to `main` contains one or more `feat:` or `feat!:` commits since the last release
- **THEN** the workflow SHALL open a release PR titled `Release v<new_version>` with the version bump commit on the `release/v<new_version>` branch
- **AND** the version written to Cargo.toml SHALL exactly match the dry-run prediction
- **AND** the PR body SHALL contain the full CHANGELOG entry for this version

#### Scenario: no conventional commits since the last release
- **WHEN** a push to `main` contains no `feat:`, `fix:`, or breaking-change commits since the last release
- **THEN** the workflow SHALL NOT open a release PR

#### Scenario: release PR already exists for this version
- **WHEN** a release PR for `release/v<new_version>` is already open (e.g., from a previous push)
- **THEN** the workflow SHALL update the existing PR's branch with the latest version bump commit (force-push the branch)
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

### Requirement: workflow tags the merge commit on the next run after a release PR is merged
On every push to `main`, the workflow SHALL detect the most recent merged release PR whose head branch matches `^release/v\d+\.\d+\.\d+$`. When such a merged PR is found, the workflow SHALL:
1. Create a git tag `v<new_version>` pointing to the merge commit SHA via the GitHub API
2. Create a GitHub Release for the tag with the release PR body (which contains the CHANGELOG entry) as the release notes
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
