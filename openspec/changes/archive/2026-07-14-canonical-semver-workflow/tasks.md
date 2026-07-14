## 1. Add `grubble --release-from-pr` flag

- [x] 1.1 In `src/main.rs`, add a `release_from_pr: Option<u32>` field to `Args` (mutually exclusive with bump modes via `conflicts_with_all = ["bump_type", "raw", "dry_run"]`)
- [x] 1.2 Add a `run_release` function that uses `gh api` shelling (no new dep) to fetch the PR, validate it is merged, extract the head branch name, parse the version from `release/v<version>`, and extract the merge commit SHA
- [x] 1.3 Emit the result as JSON (when `--output json`) or human-readable text (default)
- [x] 1.4 Add error handling: PR not found, PR not merged, branch name doesn't match `^release/v\d+\.\d+\.\d+$`, no GitHub token set
- [x] 1.5 Add 4 unit tests for the branch-name parser (`parse_release_branch`)
- [x] 1.6 Update `--help` output via clap doc comments (README CLI section update tracked under task 3.1)

## 2. Restructure `version.yml` to the canonical pattern

- [x] 2.1 Replace the existing "Bump version" step with a "Bump (dry-run)" step that runs `grubble --raw --dry-run` and captures the next version into a step output
- [x] 2.2 Add a "Detect merged release PR" step that uses `gh pr list --state merged --base main --limit 20 --json number,headRefName,mergeCommit,mergedAt | jq` to find the most recent merged release PR matching `^release/v\d+\.\d+\.\d+$`. Emit `merged`, `pr_number`, and `merge_sha` as step outputs
- [x] 2.3 Add an "Open or update release PR" step that runs only when `bump.changed == 'true'` AND `detect.merged != 'true'`. It runs `grubble --git-branch release/v<version> --preset rust --changelog --push` (NO `--tag`), then `gh pr create` (no auto-merge)
- [x] 2.4 Add a "Release merged PR" step that runs only when `detect.merged == 'true'`. It calls `grubble --release-from-pr <pr_number> --output json` to get the tag spec, then uses `gh api` to: create the `v<version>` tag, update the `v<major>` floating tag, and create the GitHub Release. Emit `released` (only true when something was actually created), `version`, and `tag_name` outputs
- [x] 2.5 Remove the existing "Push to release branch and create PR" step (replaced by task 2.3)
- [x] 2.6 Remove the `gh pr merge --auto --merge` call (no auto-merge in the canonical flow)
- [x] 2.7 Keep the existing "Clean up stale tags" step (still needed for failed-run cleanup)
- [x] 2.8 Update the version job's `outputs:` block to emit `version_changed` (from `steps.release.outputs.released`), `new_version`, `tag_name`, `pr_number` so the downstream test/build/publish jobs continue to work

## 3. Update README to document the canonical flow

- [x] 3.1 Replace the "Releasing on protected branches" section with a "Releasing on protected branches" section that explains the canonical pattern: every push to main → release PR is opened/updated → human merges → next push tags the release. Includes a reference implementation copy/paste.
- [x] 3.2 Add a short note that direct-push (`grubble --push --tag`) remains supported for users on unprotected branches
- [x] 3.3 Preserve the "Bypass token" advanced section as a footnote for users who really need to push directly to a protected branch
- [x] 3.4 CHANGELOG entry was generated automatically by the release PR (v5.2.1 changelog)

## 4. End-to-end verification

- [x] 4.1 `cargo test --all-features` passes (verified on PR #80 CI run)
- [x] 4.2 Landed as a single PR with a non-version-bump commit message (`refactor: switch version.yml to canonical release-please flow`, PR #80)
- [x] 4.3 Verified end-to-end with v5.2.1 (PR #76) and v5.2.2 (PR #79) — release PRs were opened by the workflow, the `--raw --dry-run` capture was fixed in PR #81
- [x] 4.4 Manually merged release PRs (squash for v5.2.1, merge commit for v5.2.2). The next push to main triggered the post-merge step which created the tag and GitHub Release on the merge commit
- [x] 4.5 Tags v5.2.0/v5.2.1/v5.2.2 all reachable from main on the correct merge commits; v5 floating tag on the latest (v5.2.2); GitHub releases v5.2.0/v5.2.1/v5.2.2 all published; crates.io has v5.2.2 as the default and max version
- [x] 4.6 Archive this OpenSpec change (in progress)

## 5. (Optional, follow-up) Add a `release` subcommand test suite

- [x] 5.1 Unit tests for `parse_release_branch` (4 cases: valid, missing v prefix, missing v<digit>, wrong prefix)
- [ ] 5.2 CLI integration tests for `--release-from-pr` (success, PR not merged, branch mismatch, no token) — deferred; unit tests cover the parser, and the end-to-end verification (task 4.4) covers the GitHub API integration
