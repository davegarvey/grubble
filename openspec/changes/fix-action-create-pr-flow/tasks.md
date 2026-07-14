## 1. Run bump step: emit branch output

- [ ] 1.1 In `action.yml` "Run bump" step, after the existing `previous_version` echo (around line 429), add a block that writes `branch=...` to `$GITHUB_OUTPUT`: explicit `inputs.branch` if set, otherwise `release/v${NEW_VERSION}` when `inputs.create-pr` is `true` and `NEW_VERSION != PREV`, otherwise empty.
- [ ] 1.2 Verify the block is reached on the normal bump path (not in `bump-type-only` or `dry-run` early-exit branches).

## 2. Add Push release branch step

- [ ] 2.1 Insert a new step named "Push release branch" between "Run bump" and "Create release PR".
- [ ] 2.2 Set `if:` to `steps.bump.outputs.bump_type != 'none' && inputs.create-pr == 'true' && inputs.branch == '' && steps.bump.outputs.branch != ''`.
- [ ] 2.3 In the `run:` block, set `BRANCH="${{ steps.bump.outputs.branch }}"`, then `git checkout -B "${BRANCH}"`, then `git push --set-upstream origin "${BRANCH}" --force`, then `git push --tags --force`.

## 3. Fix Create release PR step

- [ ] 3.1 Add `env:` block to "Create release PR" with `GH_TOKEN: ${{ github.token }}`.
- [ ] 3.2 Keep `BRANCH="${{ steps.bump.outputs.branch }}"` (already correct, but the value is now populated by the new block in task 1.1).

## 4. Reduce Enable auto-merge step

- [ ] 4.1 Remove the `PUSHING_TO_BRANCH` / `NEW_VERSION` / `PREV` block and the `git checkout -b` / `git push --set-upstream origin "${BRANCH}"` / `git push --tags --force` lines (the auto-generated branch push is now in the new step from task 2).
- [ ] 4.2 Remove the `if [ -n "${{ inputs.branch }}" ]; then echo "branch=${{ inputs.branch }}" >> $GITHUB_OUTPUT; fi` block (the branch output is set on the "Run bump" step in task 1.1).
- [ ] 4.3 Leave the `gh pr merge "${{ steps.pr.outputs.pr_url }}" --auto --squash` call, gated on `steps.pr.outputs.pr_url` being non-empty.

## 5. Verify the result

- [ ] 5.1 Re-read `action.yml` end-to-end to confirm step ordering: bump → push branch → PR → auto-merge, and that no step references local shell variables from a different step's `run:` block.
- [ ] 5.2 Confirm the action-level `branch` output (lines 101-103) resolves to a real value (was always empty before this change).
- [ ] 5.3 Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-features` to confirm no Rust regressions (no Rust changes expected; this is a YAML-only fix).
- [ ] 5.4 Close issue #73 with a reference to the commit SHA once the PR is merged.
