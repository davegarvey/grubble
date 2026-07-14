## 1. Add `grumble --force-push` flag

- [ ] 1.1 In `src/main.rs`, add a `force_push: bool` field to `Args` with `#[arg(long, requires = "git_branch")]` so clap enforces the constraint at the arg-parsing layer
- [ ] 1.2 In `src/git.rs`, add a `push_branch(branch: &str, force_with_lease: bool) -> BumperResult<()>` helper that wraps `git push` and conditionally adds `--force-with-lease`. Route the existing `--push` code path through this helper with `force_with_lease: false` to consolidate
- [ ] 1.3 In `src/main.rs`'s bump flow, branch on `args.force_push` after the existing `args.push` check: if `force_push` is set, call `push_branch(branch, true)`; otherwise call `push_branch(branch, false)`. The branch name comes from `--git-branch` (already validated by clap)
- [ ] 1.4 Update `--help` output via the doc comment on the new `force_push` field (clap picks it up automatically)

## 2. Update `version.yml` Open step to use `--force-push`

- [ ] 2.1 In `.github/workflows/version.yml`'s "Open or update release PR" step, add `--force-push` to the `grumble` invocation: `./target/release/grumble ... --push --force-push` (alongside the existing `--git-branch "${BRANCH}"`)
- [ ] 2.2 Replace the misleading comment "branch was force-pushed" with one that explains what actually happens: the release branch is owned by the workflow, recreated from `main` on every push, and force-pushed with `--force-with-lease` to stay in sync
- [ ] 2.3 Confirm the new step is syntactically valid YAML and the workflow still passes `actionlint` / GitHub's own validation

## 3. Update README

- [ ] 3.1 Add `--force-push` to the Usage block as a one-line entry showing it combined with `--push --git-branch` and noting the safety variant
- [ ] 3.2 Add a "Releasing" subsection under "Best Practices" that explains the release-branch force-push pattern, why `--force-with-lease` is preferred over plain `--force`, and links to `release-please` and `semantic-release` for canonical context
- [ ] 3.3 Update the existing `--release-from-pr` Action input description if needed (it should not be affected, but verify the cross-reference is still accurate)

## 4. Tests

- [ ] 4.1 Unit test: `grumble --force-push --push` (no `--git-branch`) errors with a clear message naming `--git-branch` as the required flag
- [ ] 4.2 Unit test: `grumble --force-push --git-branch release/v9.9.9` (no `--push`) succeeds the arg layer; the bump proceeds and the push is skipped (no `--push` means no push) — confirms `--force-push` only modifies the push behavior, not the bump behavior
- [ ] 4.3 Unit test (if feasible): the new `push_branch(branch, true)` call constructs the right `git push --force-with-lease origin <branch>` command. If direct assertion is too brittle (depends on how `Command` is invoked), cover via end-to-end instead
- [ ] 4.4 End-to-end (verified by the `canonical-release-workflow` spec, not directly in this change): a release branch created by a workflow run is correctly re-synced by the next workflow run after a non-bump-changing commit lands on main. This is the regression scenario from issue #89

## 5. End-to-end verification

- [ ] 5.1 Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-features` — all clean
- [ ] 5.2 Land the change as a single `fix:` PR (binary + workflow + README). The version workflow SHOULD open a v5.2.3 release PR that includes this fix along with the un-released fix: commits from #81, #82, #83, #86 (the cumulative patch)
- [ ] 5.3 Confirm: after this PR merges, the version workflow re-runs, the Open step's `--force-push` correctly re-bases the v5.2.3 release branch onto the new main HEAD (no `non-fast-forward` rejection), and the v5.2.3 release PR is updated in place
- [ ] 5.4 Merge the v5.2.3 release PR. Confirm: the next push runs the version workflow's Release step, which creates the `v5.2.3` tag on the merge commit and the GitHub Release. v5.2.3 is then fully released
- [ ] 5.5 Archive this OpenSpec change

## 6. (Optional, follow-up) Add an end-to-end test for the force-push re-sync

- [ ] 6.1 Add a CI workflow that simulates the issue #89 scenario: open a release PR, push a non-bump commit to main, verify the release branch is re-synced. This is a regression test that runs in CI on every PR, not just the v5.2.3 release
