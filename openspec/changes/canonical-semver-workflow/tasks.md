## 1. Add `grubble release` subcommand

- [ ] 1.1 In `src/main.rs`, add a new `release` variant to the `Commands` enum (or the top-level subcommand structure), with a `--from-pr <NUMBER>` flag
- [ ] 1.2 Add a `run_release` function that uses the GitHub API (via the `reqwest` crate or `gh api` shelling) to fetch the PR, validate it is merged, extract the head branch name, parse the version from `release/v<version>`, and extract the merge commit SHA
- [ ] 1.3 Emit the result as JSON (when `--output json`) or human-readable text (default)
- [ ] 1.4 Add error handling: PR not found, PR not merged, branch name doesn't match `^release/v\d+\.\d+\.\d+$`, no GitHub token set
- [ ] 1.5 Add a unit test for the version-parsing logic from a branch name
- [ ] 1.6 Update `--help` output and the README's CLI section to document the new subcommand

## 2. Restructure `version.yml` to the canonical pattern

- [ ] 2.1 Replace the existing "Bump version" step with a step that runs `grubble --dry-run` and captures the next version into a step output
- [ ] 2.2 Add a "Detect merged release PR" step that uses `gh pr list --state merged --base main --limit 10 --json number,headRefName,mergeCommit,mergedAt` to find the most recent merged release PR matching `^release/v\d+\.\d+\.\d+$`. Emit `merged=<true|false>`, `pr_number`, `merge_commit_sha`, and `version` as step outputs
- [ ] 2.3 Add an "Open or update release PR" step that runs only when `bump.changed == 'true'` AND `merged == 'false'`. It runs `grubble --tag --changelog --update-major-tag --git-branch release/v<version>` (no `--push`), then `git push --set-upstream origin release/v<version> --force`, then `gh pr create --base main --head release/v<version> --title "Release v<version>" --body "..."`. No auto-merge
- [ ] 2.4 Add a "Release merged PR" step that runs only when `merged == 'true'`. It calls `grubble release --from-pr <pr_number> --output json` to get the tag spec, then uses `gh api` to: create the `v<version>` tag, create the GitHub Release, and update the `v<major>` floating tag. Emit `version` and `tag_name` outputs
- [ ] 2.5 Remove the existing "Push to release branch and create PR" step (replaced by task 2.3)
- [ ] 2.6 Remove the `gh pr merge --auto --merge` call (no auto-merge in the canonical flow)
- [ ] 2.7 Keep the existing "Clean up stale tags" step (still needed for failed-run cleanup)
- [ ] 2.8 Update the version job's `outputs:` block to emit `version_changed`, `new_version`, `tag_name` based on the post-merge step's outputs (so the downstream test/build/publish jobs continue to work)

## 3. Update README to document the canonical flow

- [ ] 3.1 Replace the "Releasing on protected branches" section (README.md:255-329) with a "How releases work" section that explains the canonical pattern: every push to main → release PR is opened/updated → human merges → next push tags the release
- [ ] 3.2 Add a short note that direct-push (`grubble --push --tag`) remains supported for users on unprotected branches
- [ ] 3.3 Preserve the "Bypass token" advanced section as a footnote for users who really need to push directly to a protected branch
- [ ] 3.4 Update the CHANGELOG with an entry describing the workflow restructuring

## 4. End-to-end verification

- [ ] 4.1 Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-features` — all clean
- [ ] 4.2 Land the change as a single PR with a non-version-bump commit message (e.g., `refactor: restructure version.yml to canonical release-please pattern`). Verify the version workflow does NOT open a release PR for this commit (it's a `refactor:`, no version bump)
- [ ] 4.3 Open a small conventional-commit change (e.g., `docs: ...`) on a branch and merge it to main. Verify the version workflow opens a release PR
- [ ] 4.4 Manually merge the release PR (squash or merge). Verify the next push to main triggers the post-merge step which creates the tag and GitHub Release on the merge commit
- [ ] 4.5 Confirm the tag is reachable from main and the GitHub Release is published
- [ ] 4.6 Archive this OpenSpec change

## 5. (Optional, follow-up) Add a `release` subcommand test suite

- [ ] 5.1 Add CLI tests in `tests/cli_test.rs` for the `release` subcommand: success case, PR not merged, branch pattern mismatch, no token
- [ ] 5.2 Add a small unit test for the version-from-branch-name parser
