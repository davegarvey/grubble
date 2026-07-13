## 1. Rust binary: branch push support

- [x] 1.1 Add `--git-branch` CLI argument (`#[arg(long, default_value = "")]`) to `Args` struct in `src/main.rs`
- [x] 1.2 Update `git::push()` in `src/git.rs` to accept `branch: &str` — use `git push --set-upstream origin <branch>` when non-empty, existing `git push` when empty
- [x] 1.3 Update `git::push_with_force_tags()` in `src/git.rs` with the same branch-aware behavior
- [x] 1.4 Update call sites in `src/main.rs` to pass the new `--git-branch` value to `git::push()` and `git::push_with_force_tags()`

## 2. Integration tests for branch push

- [x] 2.1 Add `test_push_to_branch` test in `tests/cli_test.rs` — set up a local bare repo, run grubble with `--git-branch`, verify the branch exists on the remote with the bump commit
- [x] 2.2 Add `test_push_to_branch_with_force_tags` test — same pattern with `--update-minor-tag` to exercise the force-tag path

## 3. action.yml: new inputs and shell steps

- [x] 3.1 Add `branch` input (string, default `""`, desc: "Push the bump to this branch instead of HEAD")
- [x] 3.2 Add `create-pr` input (boolean, default `"false"`, desc: "Auto-create a PR from the branch to the default branch")
- [x] 3.3 Add `auto-merge` input (boolean, default `"false"`, desc: "Enable auto-merge with squash on the created PR")
- [x] 3.4 Add `token` input (string, default `""`, desc: "Custom token for push authentication — escape hatch for protected branches")
- [x] 3.5 Add shell step for token setup: `::add-mask::` then `git remote set-url origin https://x-access-token:$TOKEN@github.com/$REPO`
- [x] 3.6 Add validation step: fail early if `create-pr` is true and `branch` is empty
- [x] 3.7 Add validation step: fail early if `auto-merge` is true and `create-pr` is not true
- [x] 3.8 Pass `--git-branch` to the grubble binary when `branch` input is set
- [x] 3.9 Add shell step for `gh pr create` when `create-pr` is true
- [x] 3.10 Add shell step for `gh pr merge --auto --squash` when `auto-merge` is true

## 4. Documentation

- [x] 4.1 Add "Releasing on protected branches" section to README with the recommended workflow example
- [x] 4.2 Add "Bypass token (advanced)" section to README documenting the `token` input
- [x] 4.3 Document minimum required `permissions` for the caller workflow (`contents: write`, `pull-requests: write`)

## 5. Fix auto-generation chicken-and-egg in action.yml

- [x] 5.1 Auto-generate branch name `release/v<version>` when `create-pr` is true and `branch` is empty — avoids the `${{ steps.bump.outputs.version }}` bug where outputs don't exist at input evaluation time
- [x] 5.2 Remove `branch` requirement from `create-pr` validation (branch is optional now, auto-generated when missing)
- [x] 5.3 Fix README example: remove broken `branch: release/v${{ steps.bump.outputs.version }}` reference

## 6. Dogfood in `.github/workflows/version.yml`

- [x] 6.1 Replace direct binary bump with `uses: ./` (local action reference) — all the bump, branch push, PR creation, and auto-merge is now handled by the action itself
- [x] 6.2 Remove manual branch push, PR creation, and auto-merge steps (handled by the action's `create-pr` and `auto-merge` inputs)
- [x] 6.3 Update permissions: `pull-requests: write` for PR creation
- [x] 6.4 Simplify `Check version change` step to use `steps.bump.outputs.*` with Cargo.toml fallback for `skip_version_bump` mode
- [x] 6.5 Keep `Build grubble` as a compilation check (early failure detection, not a test gate)
