## Context

v5 (#61) added the `branch`, `create-pr`, `auto-merge`, and `token` inputs to `action.yml` to support releases on protected `main` branches via a temporary `release/v<version>` branch and a PR with auto-merge. The feature was shipped but the composite step wiring is broken (issue #73):

1. **Missing auth on `Create release PR` step (action.yml:431-453).** The step runs `gh pr create` with no `env:` block. In a composite action, `gh` requires `GH_TOKEN` (or `GITHUB_TOKEN`) explicitly — unlike the default `GITHUB_TOKEN` available to first-party `actions/xyz` steps. The step fails with an auth error.
2. **Out-of-scope shell variables in `Enable auto-merge` step (action.yml:455-474).** The step references `PUSHING_TO_BRANCH`, `NEW_VERSION`, `PREV` which are local to the `Run bump` step's `run:` block. Each `run:` is a fresh shell — these are empty here, so the `git checkout -b` / `git push --set-upstream` block is a no-op and the auto-generated branch is never pushed.
3. **Empty `BRANCH` in `Create release PR` (action.yml:439).** `--head "${BRANCH}"` is read from `${{ steps.bump.outputs.branch }}` — but the `bump` step never emits a `branch` output. The output is only ever written inside the `Enable auto-merge` step, which is a different step's output scope.
4. **Wrong ordering.** The branch push is in the `Enable auto-merge` step, which runs *after* `Create release PR`. `gh pr create --head X` requires `X` to exist on the remote; the PR step cannot succeed.

`action.yml:101-103` declares a `branch` action output sourced from `${{ steps.bump.outputs.branch }}`, so once bug #3 is fixed the action-level output also becomes meaningful (today it always resolves to an empty string).

`.github/workflows/version.yml:105-127` already has a working version of the same flow: bump → push branch → create PR → enable auto-merge, with `GITHUB_TOKEN` set. The fix here mirrors that pattern.

## Goals / Non-Goals

**Goals:**
- Make the `create-pr` / `auto-merge` flow functional for both auto-generated and explicit `branch` modes
- Restore correctness of the action-level `branch` output (it currently always resolves to empty)
- Mirror the working pattern in `.github/workflows/version.yml:105-127`
- Keep `push: true` only consumers unaffected (no new required step in that path)

**Non-Goals:**
- Adding new inputs or changing the contract of `create-pr`, `auto-merge`, `branch`
- Changing the binary; this is a YAML-only fix
- Adding automated tests for the composite action (no infrastructure for that exists; tests/cli_test.rs is CLI-only — this gap predates the fix and is out of scope)
- Changing `.github/workflows/version.yml` (it already works)
- Documenting the PR flow differently in `README.md` (the existing prose at README.md:298-304 already describes the post-fix behavior)

## Decisions

### 1. Set `branch` in the `Run bump` step output

Add a block at the end of the `Run bump` step (after line 429):

```bash
# Branch output: explicit input > auto-generated release/v<NEW_VERSION> when create-pr is on and a bump happened > empty
if [ -n "${{ inputs.branch }}" ]; then
  echo "branch=${{ inputs.branch }}" >> $GITHUB_OUTPUT
elif [ "${{ inputs.create-pr }}" = "true" ] && [ "$NEW_VERSION" != "$PREV" ]; then
  echo "branch=release/v${NEW_VERSION}" >> $GITHUB_OUTPUT
else
  echo "branch=" >> $GITHUB_OUTPUT
fi
```

**Why in `Run bump`:** The branch name is known as soon as the new version is known; the value is needed by both the new `Push release branch` step and the `Create release PR` step. Publishing it from `Run bump` lets both consumers read it via `${{ steps.bump.outputs.branch }}` without re-deriving the value.

**Why shell conditional over an action-level `if`:** The `Run bump` step has many early-exit branches (bump-type-only, dry-run). Deriving `branch` from the same shell context as `NEW_VERSION`/`PREV` keeps the logic in one place. Action-level conditionals can't express "branch is set OR auto-generated AND a bump happened."

**Alternatives considered:**
- *Compute `branch` in a separate pre-step* — rejected: would re-run shell logic to re-derive the version, drifting from the bump step's logic.
- *Re-derive `branch` in the PR step from inputs + outputs* — rejected: duplicates the derivation, makes the new step's conditional harder to read.

### 2. New dedicated `Push release branch` step

Insert a new step between `Run bump` and `Create release PR`:

```yaml
- name: Push release branch
  if: steps.bump.outputs.bump_type != 'none' && inputs.create-pr == 'true' && inputs.branch == '' && steps.bump.outputs.branch != ''
  shell: bash
  run: |
    BRANCH="${{ steps.bump.outputs.branch }}"
    git checkout -B "${BRANCH}"
    git push --set-upstream origin "${BRANCH}" --force
    git push --tags --force
```

**Why force-create the branch:** A previous failed run could leave a `release/v<X>` branch on the remote. `git checkout -B` (note: capital `-B`, not `-b`) force-creates the branch, then `--force` overwrites the remote. This is the exact pattern in `.github/workflows/version.yml:117-120`.

**Why `--force` on `git push`:** Same reason — robustness against stale remote state from a previous run.

**Why `git push --tags --force`:** Mirrors the binary's `push_with_force_tags` behavior in `src/main.rs:411` when `update_major_tag` or `update_minor_tag` is set. Keeps moving `v4` / `v4.1` tags consistent across runs.

**Why gated on `inputs.branch == ''`:** When the user supplies an explicit `branch`, grubble's `--git-branch` flag already pushed to it (`src/main.rs:413`). The action must not push again or it would force-push over the user's branch.

**Why gated on `steps.bump.outputs.branch != ''`:** Defensive — if the bump step set an empty `branch` output (no PR requested, no explicit branch, no bump), skip.

**Alternatives considered:**
- *Keep the push inside `Enable auto-merge` and re-order the steps* — rejected: the original problem is that the push references local vars from `Run bump`. Fixing the variable scope AND moving the step is two changes; a new dedicated step is one change with a clear, narrow responsibility.
- *Make grubble itself push the auto-generated branch* — rejected: grubble doesn't know whether a PR flow is wanted; pushing directly would defeat the "use a release branch to satisfy branch protection" purpose. The split is intentional and matches the v5 design.

### 3. Add `GH_TOKEN` to the `Create release PR` step

```yaml
- name: Create release PR
  id: pr
  if: inputs.create-pr == 'true' && steps.bump.outputs.bump_type != 'none'
  env:
    GH_TOKEN: ${{ github.token }}
  shell: bash
  run: |
    ...
    BRANCH="${{ steps.bump.outputs.branch }}"
    ...
```

`gh` in composite actions needs `GH_TOKEN` or `GITHUB_TOKEN` set explicitly. `GITHUB_TOKEN` is also valid; `GH_TOKEN` is the conventional name and matches what `gh` itself documents.

**Why `GH_TOKEN` and not `GITHUB_TOKEN`:** Both work; `GH_TOKEN` is the variable `gh` checks first in its resolution order. `.github/workflows/version.yml:107` uses `GITHUB_TOKEN` — using the other valid name here is fine because both work; the convention isn't load-bearing.

**Why `github.token` and not `secrets.GITHUB_TOKEN`:** `github.token` is the default token automatically provisioned by Actions for the workflow run. Using `secrets.GITHUB_TOKEN` is equivalent but requires an explicit `permissions:` block; `github.token` works with the default `contents: write` and `pull-requests: write` permissions documented at README.md:280-285.

**Alternatives considered:**
- *Use `actions/github-script` to call the REST API directly* — rejected: `gh pr create` is well-tested, has clean error messages, and matches the pattern at `version.yml:122`. Switching to a custom script would be a larger change with no benefit.

### 4. Reduce `Enable auto-merge` to just enable auto-merge

```yaml
- name: Enable auto-merge
  if: inputs.auto-merge == 'true' && inputs.create-pr == 'true' && steps.bump.outputs.bump_type != 'none'
  shell: bash
  run: |
    if [ -n "${{ steps.pr.outputs.pr_url }}" ]; then
      gh pr merge "${{ steps.pr.outputs.pr_url }}" --auto --squash
    fi
```

Removes the dead `PUSHING_TO_BRANCH` / `NEW_VERSION` / `PREV` block (the branch push is in step #2 above) and the redundant `inputs.branch` output write (the value is now in `steps.bump.outputs.branch`).

**Why keep this as a separate step:** The `gh pr merge --auto` call depends on `steps.pr.outputs.pr_url`, so it must run after the PR step. Combining it with the PR step would be possible but is unnecessary and would couple two distinct responsibilities.

**Why remove the `inputs.branch` echo:** It is now redundant — `steps.bump.outputs.branch` already carries the explicit or auto-generated branch.

## Risks / Trade-offs

- **[Risk] Forcing the push on the auto-generated branch could overwrite in-progress work from another run** → Mitigation: the branch name is `release/v<version>` which is unique per released version. A second run targeting the same version is a no-op anyway. Cross-version collisions are not a real concern.
- **[Risk] `--force` on `git push` could surprise users who expect `git push` safety** → Mitigation: the action is explicitly designed for a PR-based release flow where the release branch is a temporary, action-owned artifact. The `v4.9.4` `release.yml` and current `version.yml` already use `--force` for the same reason.
- **[Risk] Auto-merge enables squash-merge, which may not match the repo's default merge strategy** → Mitigation: the v5 input description (`action.yml:82-85`) explicitly says "Enable auto-merge with squash." Users who want a different strategy shouldn't set `auto-merge: true`.
- **[Trade-off] The fix ships to the `v5` floating tag, but `v5.x` action consumers on `davegarvey/grubble@v5` will pick it up automatically on next workflow run.** This is the intended behavior of floating tags and matches the v5 release pattern.
- **[Trade-off] No automated test for the composite action.** Pre-existing gap. The fix is verified by reading and by a downstream user exercising the flow against a real protected branch.

## Migration Plan

1. Land the `action.yml` edits on a branch.
2. Manually verify by:
   - Inspecting the diff against this design.
   - Reading the final YAML top-to-bottom to confirm step ordering (bump → push → PR → auto-merge) and that no other step references local shell variables from a different step.
3. The release is `v5.2.0` is already on `main` and the `v5` floating tag will pick up the new `action.yml`. Consumers using `davegarvey/grubble@v5` will receive the fix on their next workflow run.
4. No CHANGELOG entry is required for a patch-level action-only fix, but the issue is closed by referencing the commit SHA in a comment on issue #73.

**Rollback strategy:** Revert the commit on `main`. The `v5` floating tag re-points to the previous commit on the next composite-action resolution. No data is at risk because the buggy flow was a no-op (`gh pr create` failed, so no PR was ever created). Downstream consumers on `@v5.2.0` (the current tag) would need to bump back to a prior tag to revert; not expected to be necessary.

## Open Questions

_Resolved during design:_
- *Should the new `Push release branch` step use `git checkout -b` (fail if exists) or `git checkout -B` (force-create)?* `git checkout -B` — robustness against stale remote branches from prior failed runs.
- *Should the new step also handle `auto-merge`?* No — auto-merge is its own step that depends on the PR URL.
- *Should the `branch` echo in the old `Enable auto-merge` step be kept as a fallback?* No — `steps.bump.outputs.branch` is the single source of truth after the fix; keeping a duplicate write invites drift.
