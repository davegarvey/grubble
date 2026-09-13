## Why

Grubble currently returns `"none"` for `--bump-type` and refuses to bump when no tag is reachable from HEAD. This blocks first-run users who have a fresh repo with commits but no tags. The user must manually create an initial tag before grubble can do anything useful. An `--initial-version` flag lets users explicitly declare "this is the first release — treat X.Y.Z as the previous version and scan all commits."

## What Changes

- Add `--initial-version <SEMVER>` CLI flag to grubble
- When set and no tag is reachable from HEAD: use the initial version as the baseline, scan all commits to determine the bump, and produce the correct next version
- When set and a tag IS reachable from HEAD: error out (contradictory state)
- Works with `--bump-type`: outputs the bump type relative to the initial version
- Works with normal bump flow (`grubble --initial-version 0.1.0 --tag`): bumps version, creates files, creates the first tag
- Works with `--raw` / `--dry-run`: outputs the new version without modifying files
- When no tag exists and `--initial-version` is NOT set: current behavior (return `"none"`, no bump)

## Capabilities

### New Capabilities
- `initial-version-flag`: The `--initial-version` flag that provides a release baseline when no git tag exists.

### Modified Capabilities
- `bump-base-validation`: The "No tags exist (first release)" scenario currently states the program should "proceed with the normal bump flow (first release path)" but no such path exists. This change defines and implements that path, gated behind `--initial-version`.

## Impact

- `src/main.rs`: Add `--initial-version` arg to `Args` struct. Update `run_bump_type()` and `run()` to check for the flag when `get_last_tag()` returns `None`.
- `src/git.rs`: Add `get_all_commits()` function for scanning full commit history (only used when `--initial-version` is explicitly set).
- `tests/cli_test.rs`: Add integration tests for the flag.
- No new dependencies. No breaking changes to existing behavior (the flag is opt-in).
