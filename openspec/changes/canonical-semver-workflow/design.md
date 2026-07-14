## Context

Grubble's `version.yml` workflow (the workflow that releases grubble itself) was historically designed around pre-tagging the release branch commit. This is non-canonical: `semantic-release` (23.9k ⭐) and `release-please` (7.2k ⭐, Google-maintained) both create the tag **on the main merge commit**, not on a release branch. The pre-tagging approach has been the source of repeated issues:

- v5.2.0 release: `v5.2.0` tag was orphaned because the squash-merge created a new commit on main that didn't have the release branch as a parent. Required manual re-tagging.
- v5.2.1 release: same root cause. Required the `Recreate orphaned release tags` workaround (#77).
- Bot-created release PRs enter `action_required` state for CI approval (GitHub security model: `GITHUB_TOKEN`-created PRs always need a human to approve CI). This required a one-off `gh run rerun` for every release.
- Auto-merge pushes (attributed to `github-actions[bot]`) do not trigger subsequent workflow runs. The cycle resets only on the next human push to main.
- The fix in #78 (use `--merge` instead of `--squash`) keeps the tag reachable but doesn't address the bot approval issue, and the tag is still on a release branch commit — unconventional.

The canonical pattern (release-please's model): on every push to main, the workflow computes the next version from conventional commits. If a release is warranted, it opens or updates a release PR. The PR contains the version bump commit and the CHANGELOG update. **A human reviews and merges the PR.** The next workflow run detects the merged release PR, creates the `v<version>` tag on the main merge commit via the GitHub API, creates a GitHub Release, and triggers build/publish.

This pattern is the industry standard for a reason: it's correct (tag on the right commit by construction), it's secure (humans approve releases), and it doesn't depend on auto-merge quirks. The grubble repo's own release workflow should demonstrate this.

## Goals / Non-Goals

**Goals:**
- Restructure `version.yml` to match the canonical release-please pattern (release PR → human merge → tag on main commit on next run)
- No more auto-merge; the release PR stays open until a human merges it
- The `v<version>` tag is created on the merge commit (or squash commit) on main, not on a release branch
- Both squash-merge and merge-commit work (release-please's "both work" guarantee)
- Future workflow runs on subsequent pushes to main don't require re-tagging or workarounds
- Add a small `grubble release` subcommand that the workflow calls to drive the post-merge release step (so the logic lives in the binary, not in shell scripts)
- Document the canonical flow in `README.md` as the recommended pattern
- The existing direct-push CLI style (`grubble --push --tag`) keeps working for users with unprotected branches

**Non-Goals:**
- Changing grubble's CLI bump logic (conventional commit parsing, version resolution, file updates, tag creation are all unchanged)
- Changing the `action.yml` (the composite action) — its auto-merge with `--squash` remains available as a separate, action-only style
- Adding new CLI flags to grubble's bump step (the bump step continues to support `--tag --update-major-tag` etc. for direct-push use)
- Replacing `gh pr create` with `peter-evans/create-pull-request` or similar (the existing `gh` CLI works)
- Changing how `release.yml`'s downstream jobs (test, create-release, build-release, publish) are triggered — they continue to run based on outputs from the version job

## Decisions

### 1. Two distinct release steps in the workflow

The version job has two responsibilities:
- **Pre-merge**: open or update a release PR with the version bump commit
- **Post-merge**: when a release PR is detected as merged, create the tag on the merge commit and emit a release

These run in two different workflow steps so the gating is clean:

```yaml
- name: Open or update release PR
  if: steps.bump.outputs.changed == 'true' && steps.bump.outputs.merged == 'false'
  # creates or updates the release PR

- name: Release merged PR
  if: steps.bump.outputs.merged == 'true'
  # detects the merged PR, tags the merge commit, creates the release
```

The `merged` output is computed by a small step that looks up the most recent merged release PR via the GitHub API and extracts its merge commit SHA.

**Why two steps:** The pre-merge step must not run when a release PR is already merged (else it would re-open the PR). The post-merge step must not run when there's no merged release PR to process. A single combined step would need complex internal logic; two steps with mutually-exclusive `if:` conditions is clearer.

### 2. The Bump step uses `--dry-run` (or a new "no-tag, no-push" mode)

Currently the Bump step runs `./target/release/grubble --tag --release-notes --changelog --update-major-tag` which **creates local commits and tags**. In the canonical flow, the Bump step should only **compute the next version** — no commits, no tags, no pushes. The actual commit and tag creation happens via:

- The "Open or update release PR" step runs `grubble` **with** `--tag --changelog` and a non-push `--git-branch` to create the release branch with the version bump commit and CHANGELOG
- The "Release merged PR" step creates the **final** tag on the merge commit via the GitHub API

To avoid adding a new "compute only" mode to grubble's CLI, the Bump step uses the existing `--dry-run` flag, which already returns the next version without modifying files (it exits 0 and prints the version). The version is captured into a step output and used by both downstream steps.

**Why dry-run over a new flag:** `--dry-run` is the canonical concept of "compute without side effects" and grubble already implements it. Adding a separate flag for "compute the next version" would be redundant.

**Caveat:** `--dry-run` exits 0 always. The `changed` output of the Bump step is computed by comparing the dry-run version to the current Cargo.toml version. If the dry-run returns the same version as Cargo.toml, no bump is needed. This logic is already present in the existing "Check version change" sub-block — we adapt it to read from dry-run.

### 3. Tag and release creation via the GitHub API (not `git tag`)

The "Release merged PR" step uses `gh api` to:
- Get the merge commit SHA of the most recent merged release PR
- Create a git tag via `POST /repos/{owner}/{repo}/git/refs` pointing to the merge commit
- Create a GitHub Release via `POST /repos/{owner}/{repo}/releases` with the tag name and the CHANGELOG entry as the body
- Update the `v<major>` floating tag to point to the same commit

**Why the API and not `git tag`:** `git tag` from inside the workflow would tag the local commit (a checkout of main), but the workflow checks out `main` at the start of the run — which is BEFORE the merge commit if the workflow was triggered by a different event. Using the GitHub API is robust to the workflow run's checkout state and ensures the tag is created atomically on the merge commit.

**Why a `release` subcommand at all:** A small `grubble release --from-pr <num> --pr-repo <repo>` subcommand handles the "find the merged release PR, extract the version, print the merge commit SHA and tag name" logic. The workflow uses the printed values to drive `gh api` calls. This keeps the version-detection logic in the binary (where the rest of grubble's logic lives) and the shell glue minimal.

### 4. Detect merged release PR by branch name pattern

A merged release PR is identified by:
- A PR that was merged (not closed) into `main`
- The PR's head branch is `release/v<version>` (matches the `^release/v\d+\.\d+\.\d+$` regex)
- The PR is the most recent merged release PR (sorted by merge time)

The detection is done by a small step that uses `gh pr list --state merged --base main --json number,headRefName,mergeCommit,mergedAt` and picks the most recent matching PR.

**Why a list-and-filter approach:** GitHub's search API doesn't support regex on branch names. Listing the recent merged PRs and filtering client-side is straightforward and bounded (the API returns up to 100 by default, which covers the realistic window).

### 5. No CHANGELOG.md or Cargo.toml change in the release commit

The release commit on the release branch contains the version bump (Cargo.toml) and the CHANGELOG.md update. This is the **only** commit on the release branch. When the human merges it (squash or merge), the merge commit on main contains these file changes. The tag is then created on this merge commit.

**Why no extra commit:** The release commit is the version bump. Adding a merge commit on top (for merge merges) or a squash commit (for squash merges) is the user's choice. Either way, the tag points to the main commit that contains the release.

### 6. The `v<major>` floating tag is updated by the post-merge step

When the release is created, the `v<major>` floating tag is updated to point to the same merge commit. This happens in the same "Release merged PR" step, via a second `gh api` call (`PATCH /repos/{owner}/{repo}/git/refs/tags/v<major>`).

**Why update the floating tag in the post-merge step:** Pre-creating `v<major>` on the release branch (the current approach) has the same orphaning risk. Updating it on the merge commit keeps the floating tag on main, consistent with the canonical pattern.

## Risks / Trade-offs

- **[Risk] The release PR needs a human to merge, breaking the "fully automatic" desire from the user** → Mitigation: this is the canonical best practice. A human-in-the-loop is the security model: a release is a high-impact event and should be reviewed. The user's "fully automatic" goal is served by removing the manual CI re-run, not the PR review.

- **[Risk] If a release PR is closed without merging (e.g., the version was wrong), the next workflow run won't tag anything** → Mitigation: the post-merge step only runs when a merged release PR is detected. Closed PRs are ignored. If the user wants to release a different version, they push a new conventional commit and a new release PR is opened.

- **[Risk] Bot approval gating still applies for the release PR's CI** → Mitigation: the first CI run on the bot-created release PR may be in `action_required` state. The user (Dave) approves it once. Subsequent PRs from `github-actions[bot]` are blocked the same way — but with the canonical pattern, the release PR is only created when a release is needed (not on every push), so the user intervention is less frequent.

- **[Risk] `--dry-run` doesn't write the Cargo.toml version, so the Bump step's "is a bump needed" check requires comparing dry-run output to Cargo.toml** → Mitigation: the workflow already does this comparison (line 95-103 of the existing version.yml). We adapt the check to read from dry-run output.

- **[Trade-off] The version.yml workflow becomes more complex (more steps, more conditional logic)** → Trade-off worth it for the canonical, correct behavior. The complexity is in the workflow glue, not the bump logic.

- **[Trade-off] Grubble's `release` subcommand is added to the CLI surface** → Minimal: it just exposes "find the merged release PR for this version" — a thin convenience. Users can ignore it and use the binary directly.

## Migration Plan

1. Land the new workflow as a single PR. The PR itself is a non-version-bump change (e.g., `refactor: restructure version.yml to canonical release-please pattern`). It will trigger the version workflow, which will detect the change as a `refactor:` (no version bump) and not open a release PR. (If the version.yml change is interpreted as needing a bump, the workflow opens a release PR; the user reviews and merges it; the new flow then handles the release.)

2. Verify the new flow end-to-end by making a small conventional commit (e.g., `docs: clarify release flow in README`) and observing:
   - The version workflow opens a release PR
   - The user merges it
   - The next push (or the user manually re-runs the version workflow) creates the tag and GitHub Release

3. Document the new flow in `README.md`. The existing "Releasing on protected branches" section is rewritten to describe the canonical pattern. The "Bypass token" advanced section is preserved as an alternative for users who want direct-push.

4. Rollback: if the new flow has a bug, revert the version.yml commit. The previous flow (PR #78's merge-commit approach) still works. No data is at risk because the new flow only creates new tags/releases; it doesn't delete anything.

## Open Questions

_Resolved during design:_

- *Where does the `v<major>` floating tag get updated?* In the post-merge step, on the main merge commit. Same as the release tag.
- *What if multiple release PRs are open?* The post-merge step picks the most recently merged one. Older merged PRs that haven't been tagged are ignored — they were either re-releases or errors.
- *What if the release PR was squash-merged?* The tag points to the squash commit. Either way (squash or merge), the tag is on the right commit on main. The release notes in the GitHub Release use the squash commit's diff (one commit) which is cleaner anyway.
- *What if a release PR is merged but the version workflow isn't re-run automatically?* The next push to main (e.g., another commit) re-runs the workflow and tags the release. Or the user can manually re-run via `workflow_dispatch`.
