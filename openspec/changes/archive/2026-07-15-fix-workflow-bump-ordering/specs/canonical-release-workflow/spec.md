## MODIFIED Requirements

### Requirement: version.yml opens a release PR on every push that warrants a bump

The workflow steps SHALL run in this order: (1) **Detect merged release PR**, (2) **Release merged PR** (create tag + GitHub Release), (3) **Bump (dry-run)** (compute next version), (4) **Open or update release PR** (create/update the branch). The Bump step MUST run AFTER the Release step so that any tag just created by the Release step is visible to the dry-run analysis. Without this ordering, the dry-run would re-analyze commits already included in the just-released tag, producing a stale next-version.

The Release step is idempotent and uses the GitHub API (`gh api`). The Bump step runs for every push (unless `skip_version_bump` is set). The Open step is gated on the Bump step's `changed` output. This ordering handles the "quiet period" correctly: if v5.2.2 was released long ago and a new `fix:` lands on main, the Detect step still finds the old v5.2.2 release PR (`merged=true`), the Release step is a no-op (tag exists), and after the Bump step computes v5.2.3, the Open step opens a fresh release PR for v5.2.3.

The version SHALL be written using `grubble --release-version`, which writes the exact dry-run version without forward-bump or sync logic. The git tag SHALL NOT be created on the release branch — the tag is created on the main merge commit after the PR is merged.

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

### Requirement: Bump step fetches tags before dry-run

The Bump step SHALL run `git fetch origin --tags --force` before invoking grubble's dry-run. This ensures tags created by the preceding Release step (via the GitHub API) are visible to the local repository, preventing the dry-run from using a stale last-tag. This fetch is required even though the checkout step uses `fetch-tags: true` because the Release step creates tags via the GitHub API, not through git.

#### Scenario: Release step creates a new tag moments before Bump step
- **GIVEN** the Release step just created tag `v5.3.2` on the merge commit
- **WHEN** the Bump step runs `grubble --dry-run --raw`
- **THEN** `git fetch origin --tags --force` SHALL have been called before grubble
- **AND** grubble SHALL find `v5.3.2` as the latest tag via `git describe --tags`
- **AND** the dry-run output SHALL reflect the correct next version after `v5.3.2`

The Open step SHALL also run `git fetch origin --tags --force` before invoking `grubble --release-version --changelog` (defence in depth — the dry-run already fetched, but the CHANGELOG generation also depends on `get_last_tag()`).

### Requirement: workflow cleans up unreachable tags before bump

The "Clean up stale tags" step SHALL run before any tag-dependent analysis. It SHALL remove any local `v*` tag that is not reachable from `main` (i.e., the tag points to a commit that is not an ancestor of `main`). This prevents old failed runs from leaving orphan tags that interfere with version detection. It runs before the Release step so that stale tags do not interfere with the dry-run's `git describe --tags` computation.

#### Scenario: orphan tag from a previous failed run
- **WHEN** a previous failed run left a `v5.2.0` tag on a release branch commit that was never merged
- **THEN** the stale tag cleanup SHALL remove the local `v5.2.0` tag so the next bump step computes the correct next version

## REMOVED Requirements

### Requirement: Open step fetches tags before running grubble
**Reason**: Replaced by "Bump step fetches tags before dry-run" — the tag fetch is now the responsibility of the Bump step (which is the first step that depends on accurate tag state). The Open step still fetches as defence-in-depth, but the normative requirement belongs to the Bump step.
**Migration**: The fetch command remains in the Open step's implementation as defence-in-depth, but the spec-level guarantee is that the Bump step provides an up-to-date tag view.
