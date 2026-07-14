## Why

Grubble's own release workflow (`.github/workflows/version.yml`) is structurally different from how canonical semver tools work. The mainstream tools (`semantic-release`, `release-please`) all use a "release on the main commit" model where the tag is created on the merged commit, not pre-tagged on a release branch. Grubble's version.yml pre-tags the release branch commit, then merge-commits to main — workable but unconventional, and it inherits GitHub's auto-merge-bot limitations (CI approval gating, subsequent workflow runs not triggered by bot pushes). The fix is to restructure the workflow to match the canonical pattern: open a release PR, let a human merge it, then create the tag on the main merge commit on the next workflow run.

## What Changes

- **Restructure `.github/workflows/version.yml`** to match the canonical release-please pattern:
  - The "Bump version" step runs `grubble` **without** `--tag` and **without** `--push`. The version bump commit is only made when the release PR is being opened/updated.
  - A new "Open or update release PR" step creates (or updates) a PR with the version bump commit on a `release/v<version>` branch. The PR is opened with `gh pr create` (no auto-merge).
  - A new "Release on merge" step runs on every push to main. It detects the most recent merged release PR, creates the `v<version>` git tag on the merge commit via the GitHub API, creates a GitHub Release, and triggers the existing build/publish job chain.
  - The existing "Push to release branch and create PR" + `gh pr merge --auto --merge` pattern is removed. Auto-merge is no longer used.
  - The "Clean up stale tags" step stays — it still cleans up tags from failed prior runs.
- **Add a `release` subcommand to grubble** that takes a merged release PR (PR number) and a version, and emits the version + tag name. The workflow uses this to drive tag creation. This is small (~50 LOC) and keeps the logic in the binary rather than scattered across shell scripts.
- **Update `README.md`** to document the canonical flow as the recommended pattern. The existing "Releasing on protected branches" section is replaced with a clear "How releases work" section explaining the release-please-style flow. The "Bypass token" advanced section is preserved.
- **The `v5` floating tag and the conventional commit analysis** work unchanged. The `--git-branch` and `--push` CLI flags remain for users who want the direct-push style on unprotected branches. This is a workflow change, not a CLI change to the bump logic.

No breaking changes to grubble's CLI. The `action.yml` (the composite action) is unchanged — its existing `--squash` behavior remains as an alternative for action consumers.

## Capabilities

### New Capabilities
- `canonical-release-workflow`: Defines the end-to-end release workflow that grubble's own repo follows. The workflow opens a release PR on every push to main that triggers a version bump, waits for a human to merge it, and on the next run creates the tag + GitHub Release on the main merge commit. This is the documented, recommended pattern.
- `release-subcommand`: A new `grubble release` subcommand that, given a version and an optional merged release PR reference, returns the version and tag name (and in the future may perform the post-merge release steps). The workflow calls this to determine what to tag.

### Modified Capabilities
_None — the bump logic in `src/main.rs` is unchanged. The conventional commit parsing, the version resolution, and the tag/branch management are unaffected. Only the orchestration in `version.yml` changes._

## Impact

- **`.github/workflows/version.yml`**: significant restructuring. The auto-merge flow is removed. New steps for opening/updating a release PR and for post-merge release detection. Estimated ~80 LOC changed.
- **`src/main.rs`**: add a new `release` subcommand that takes a version and emits the tag info. No changes to the bump logic. Estimated ~50 LOC added.
- **`README.md`**: rewrite the "Releasing on protected branches" section. ~100 lines changed.
- **Action consumers (`action.yml`)**: unchanged. The action's `--squash` auto-merge behavior remains available for users who want that style.
- **Direct binary consumers (`grubble --push --tag`)**: unchanged. The CLI flags still work the same way.
- **Future v5.2.3 release**: this change lands as a single fix commit, which the version.yml workflow will pick up and release as a patch. Demonstrates the new flow end-to-end.
