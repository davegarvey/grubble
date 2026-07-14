## Why

The `version.yml` workflow's `Open or update release PR` step is supposed to keep the release branch (`release/v<version>`) in sync with `main` on every push — the same pattern used by [`googleapis/release-please`](https://github.com/googleapis/release-please) (7.2k ⭐, Google-maintained) and the release-please-style plugin for `semantic-release`. The release branch is owned by the workflow, and the workflow's job on every push is: (1) recompute the next version, (2) if it changed, refresh the release branch + PR with the latest bump commit. The branch is *expected* to be force-overwritable because the workflow is the sole writer.

The current implementation calls `grumble --push` which uses `git push` (no `--force`) under the hood. The comment in the step even claims "branch was force-pushed", but the implementation does not actually force-push. When the workflow re-runs for a push that is *not* the push that originally created the release branch, the local branch is created from the new `main` HEAD while the remote release branch is based on the previous `main` HEAD. They share the same bump commit content but have different parents, so the push is non-fast-forward and is rejected:

```
! [rejected]        release/v5.2.3 -> release/v5.2.3 (non-fast-forward)
error: failed to push some refs to 'https://github.com/davegarvey/grubble'
hint: Updates were rejected because the tip of your current branch is behind
hint: its remote counterpart. ...
```

This was the failure mode in run 29331773679 (the version.yml run after PR #87 merged). The release branch was left at its previous out-of-date state, and the workflow run failed. The release PR (PR #88) then showed "branch is out of date" in the UI and required a manual `git rebase` + force-push to unblock.

The comment in the step is misleading in two ways: (1) it claims a force-push happened when none did, and (2) it implies the step is always running, but the step is only re-run when `changed=true` — non-bump-changing pushes between releases still leave the release branch stale.

## What Changes

- **New CLI flag `--force-push`** in the `grumble` binary. When combined with `--git-branch`, the push uses `git push --force-with-lease` instead of `git push`. This matches `release-please`'s behavior (which uses `--force-with-lease` for its release branches for the same safety reason: avoid clobbering concurrent updates while still allowing the workflow to be the sole writer). The `--force-with-lease` variant protects against the failure mode of plain `--force` — if a human or another workflow has updated the branch since the workflow checked it out, the push fails rather than silently overwriting.
- **The flag is gated to `--git-branch` context.** Force-pushing to the current branch (`main`, etc.) is dangerous and out of scope; the flag is only meaningful when pushing to a specific named release branch. This matches `release-please`'s design (which only force-pushes the release branch, never main). If `--force-push` is set without `--git-branch`, `grumble` errors out with a clear message.
- **`version.yml` Open step** switches from `./target/release/grumble ... --push` to `./target/release/grumble ... --push --force-push` (and uses `--git-branch "${BRANCH}"` which it already does). The misleading "branch was force-pushed" comment is replaced with one that explains what actually happens.
- **No new flag semantics for the bump step's general `--push`**. The existing `--push` (regular non-fast-forward) behavior is preserved for non-`--git-branch` use cases. Force-push is an opt-in for the release-branch workflow — same as how `release-please` does it: the default push is non-force, and the release-branch-push is a separate code path that uses `--force-with-lease`.

## Capabilities

### New Capabilities

- `release-branch-force-push`: Defines the contract for `grumble --force-push` — when set, the push to the named branch uses `git push --force-with-lease`; when set without `--git-branch`, the binary errors out. Establishes that the flag is the only safe way to keep a workflow-owned release branch in sync with `main` across multiple workflow runs, matching the canonical release-please pattern.

### Modified Capabilities

- None. The `canonical-release-workflow` capability's "Open or update release PR" scenario already implies force-push behavior (the branch is owned by the workflow and should be syncable from any main state). The bug was an implementation gap, not a spec gap.

## Impact

- **`grumble` binary users**: new flag `--force-push` is opt-in. Default behavior (regular `--push`) is unchanged. No existing scripts are affected unless they explicitly use the new flag.
- **`version.yml` workflow**: the Open step's push is no longer rejected. Release branches stay in sync with `main` across bump-changing and non-bump-changing pushes (as long as the workflow re-runs, which it does on every push to `main`).
- **Tests**: new test asserting that `--force-push` without `--git-branch` errors; the flag's wiring to `git push --force-with-lease` is verified by the `canonical-release-workflow` end-to-end spec (release PR's release branch stays in sync across multiple pushes to main).
- **`action.yml`**: unchanged. The Action does not have a release-branch-push flow; it uses `--branch` + `create-pr` for the bump + push. The `release-from-pr` mode is read-only. No new Action input is needed.
- **No version-bump coupling**: the change is itself a `fix:` commit, but it should land in the same v5.2.3 release as the un-released fix: commits from #81, #82, #83, #86 (the cumulative patch). The v5.2.3 release PR will pick up this change via the Open step's force-push, after this PR merges to main.
