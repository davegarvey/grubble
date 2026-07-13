## Why

Grubble's `push: true` pushes the version bump commit directly to `HEAD` (the current branch). On repositories with branch protection rules (required PRs, status checks), `GITHUB_TOKEN` cannot bypass these rules, causing the push to fail with `GH006: Protected branch update failed`. This blocks all users with protected branches from using `push: true`.

The industry standard among versioning tools (release-please, semantic-release) is to push to a non-protected branch and open a release PR, rather than pushing directly to the default branch.

## What Changes

- **`branch` input** — allows pushing the bump commit to a configurable branch (e.g. `release/v0.35.0`) instead of always pushing to `HEAD`
- **`create-pr` input** — after pushing, automatically opens a PR from the branch to the default branch using `gh pr create`
- **`auto-merge` input** — enables auto-merge with squash on the created PR, so it merges when checks pass
- **`token` input** — optional escape hatch to set a custom token (PAT / GitHub App) for git push authentication, allowing bypass of branch protection when needed
- Refactor of `git.rs` to accept a branch parameter in `push()` and `push_with_force_tags()`
- New CLI arg `--git-branch` in the Rust binary
- Documentation in README with the recommended protected-branch workflow

No breaking changes — existing `push: true` behavior is preserved when no new inputs are provided.

## Capabilities

### New Capabilities
- `branch-push`: Pushing the version bump commit to a user-specified branch instead of HEAD. Includes the `branch` input in action.yml, the `--git-branch` CLI arg, and updated git operations in the Rust binary.
- `pr-creation`: After pushing, automatically creating a PR from the branch to the default branch, with optional auto-merge. Includes `create-pr` and `auto-merge` inputs, implemented via `gh` CLI shell steps in action.yml.
- `token-auth`: Setting a custom token for git push authentication. Includes the `token` input, `::add-mask::` for security, and `git remote set-url` in action.yml shell steps.

### Modified Capabilities
- *(none — existing capabilities are unchanged)*

## Impact

- **`src/git.rs`**: `push()` and `push_with_force_tags()` signatures change to accept a `branch: &str` parameter
- **`src/main.rs`**: new `--git-branch` CLI argument added
- **`action.yml`**: 4 new inputs (`branch`, `create-pr`, `auto-merge`, `token`) plus new shell steps for PR creation and token setup
- **`README.md`**: new "Protected branches" section with recommended workflow
- **`tests/cli_test.rs`**: new tests for branch push operations
