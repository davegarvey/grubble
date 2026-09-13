## 1. Add `grubble --force-push` flag

- [x] 1.1 In `src/main.rs`, add a `force_push: bool` field to `Args` with `#[arg(long, requires = "git_branch")]` so clap enforces the constraint at the arg-parsing layer
- [x] 1.2 In `src/git.rs`, add a `push_branch(branch: &str, force_with_lease: bool) -> BumperResult<()>` helper that wraps `git push` and conditionally adds `--force-with-lease`. Route the existing `--push` code path through this helper with `force_with_lease: false` to consolidate
- [x] 1.3 In `src/main.rs`'s bump flow, branch on `args.force_push` after the existing `args.push` check: if `force_push` is set, call `push_branch(branch, true)`; otherwise call `push_branch(branch, false)`. The branch name comes from `--git-branch` (validated before the push path)
- [x] 1.4 Update `--help` output via the doc comment on the new `force_push` field (clap picks it up automatically)

## 2. Update `version.yml` Open step to use `--force-push`

- [x] 2.1 Implemented in PR #90. The later `--release-version` workflow path performs the equivalent direct `git push --force-with-lease` without invoking the CLI push path.
- [x] 2.2 Replace the misleading comment "branch was force-pushed" with one that explains what actually happens: the release branch is owned by the workflow, recreated from `main` on every push, and force-pushed with `--force-with-lease` to stay in sync
- [x] 2.3 Confirm the new step is syntactically valid YAML and the workflow still passes `actionlint` / GitHub's own validation

## 3. Update README

- [x] 3.1 Add `--force-push` to the Usage block as a one-line entry showing it combined with `--push --git-branch` and noting the safety variant; the reference now lives in `docs/cli.md` after PR #95.
- [x] 3.2 Add a "Releasing" subsection under "Best Practices" that explains the release-branch force-push pattern, why `--force-with-lease` is preferred over plain `--force`, and links to `release-please` and `semantic-release` for canonical context
- [x] 3.3 Update the existing `--release-from-pr` Action input description if needed (it should not be affected, but verify the cross-reference is still accurate)

## 4. Tests

- [x] 4.1 Unit test: `grumble --force-push --push` (no `--git-branch`) errors with a clear message naming `--git-branch` as the required flag
- [x] 4.2 Unit test: `grumble --force-push --git-branch release/v9.9.9` (no `--push`) succeeds the arg layer; the bump proceeds and the push is skipped (no `--push` means no push) — confirms `--force-push` only modifies the push behavior, not the bump behavior
- [x] 4.3 Manual review confirms `push_branch(branch, true)` constructs `git push --force-with-lease --set-upstream origin <branch>`; the helper is covered by the CLI path and the merged workflow change.
- [x] 4.4 End-to-end regression from issue #89 was addressed by PR #90; issue #89 is closed.

## 5. End-to-end verification

- [x] 5.1 Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-features` — all clean
- [x] 5.2 Land the change as a single `fix:` PR (binary + workflow + README). PR #90 landed the binary, workflow, and documentation changes.
- [x] 5.3 Confirm: after this PR merges, the version workflow re-runs, the Open step's `--force-push` correctly re-bases the v5.2.3 release branch onto the new main HEAD (no `non-fast-forward` rejection), and the v5.2.3 release PR is updated in place
- [x] 5.4 Merge the v5.2.3 release PR. Confirm: the next push runs the version workflow's Release step, which creates the `v5.2.3` tag on the merge commit and the GitHub Release. v5.2.3 was released successfully.
- [x] 5.5 Archive this OpenSpec change

## Deferred follow-up

- Add a CI workflow that simulates the issue #89 scenario on every pull request; the production workflow fix shipped in PR #90, but no dedicated simulator was added.
