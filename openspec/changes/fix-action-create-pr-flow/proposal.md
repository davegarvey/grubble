## Why

The `create-pr` and `auto-merge` inputs added in v5 (#61) are wired up in `action.yml` but the implementation is broken end-to-end: `gh pr create` is invoked without `GH_TOKEN`/`GITHUB_TOKEN`, the auto-generated branch is never pushed because the `Enable auto-merge` step references local shell variables that are out of scope, the branch name that `gh pr create --head` uses is empty because it's read from a step output that is never set, and the push happens *after* the PR is supposed to be created so the PR step cannot succeed. Net effect: the documented PR flow (`create-pr: true` + `auto-merge: true`) is unusable in v5.x.

## What Changes

- The `Run bump` step in `action.yml` emits a `branch` step output: the explicit `inputs.branch` value when set, otherwise an auto-generated `release/v<version>` when `create-pr` is true and a bump occurred, otherwise empty.
- A new `Push release branch` step runs after `Run bump` and only when the branch was auto-generated (no explicit `inputs.branch`). It force-creates the branch, pushes it, and pushes tags — matching `.github/workflows/version.yml`.
- The `Create release PR` step sets `GH_TOKEN: ${{ github.token }}` and reads `BRANCH` from `steps.bump.outputs.branch` (which is now populated).
- The `Enable auto-merge` step is reduced to just enabling auto-merge on the created PR; the dead branch-push block referencing out-of-scope variables is removed.
- Documented step ordering becomes: bump → push branch (if auto-generated) → create PR → enable auto-merge.

No binary changes; no breaking changes for direct binary users; no new inputs.

## Capabilities

### New Capabilities
- `action-create-pr-flow`: Defines how the GitHub Action's `create-pr` and `auto-merge` inputs coordinate across composite steps. Establishes that (a) `Run bump` declares the branch as a step output, (b) an auto-generated branch is pushed before PR creation, (c) the PR step authenticates with `GH_TOKEN`/`GITHUB_TOKEN`, and (d) auto-merge is enabled only after the PR exists. Covers all three modes: auto-generated branch, explicit `branch` input, and no-PR (`push: true` only).

### Modified Capabilities
_None — the only existing capabilities (`bump-base-validation`, `preset-aware-version-detection`) are unrelated._

## Impact

- **`action.yml`** — the only file edited. Composite step ordering changes; the action-level `branch` output now resolves to a real value instead of always being empty.
- **Action consumers** using `create-pr: true` (with or without `auto-merge: true`) — the flow now works. No input changes.
- **Action consumers** using `push: true` only — unaffected; the new step is skipped.
- **`README.md`** — no changes required; the documented behavior (`create-pr` opens a PR from the auto-generated `release/v<version>` branch) is what is being implemented.
- **No binary release required**; the v5.2.0 release on `main` is sufficient. The `v5` floating tag will pick up the action fix on its next composite-action resolution.
- **No test infrastructure changes**; the existing `tests/cli_test.rs` is CLI-only. The composite action has no automated tests (this predates the fix); verification is by manual smoke test or by the next downstream user who hits the issue.
