## 1. Test Infrastructure

- [x] 1.1 Revert `get_commits_since_tag` back to scanning all commits when last_tag is None (undo earlier fix — the safety will move to the callers)
- [x] 1.2 Keep the test `test_bump_type_no_tag_returns_none` as-is (it validates the no-tag-without-flag behavior)
- [x] 1.3 Fix `get_grubble_bin()` priority (already done — prefer local build over global PATH)

## 2. Core: CLI arg and git helper

- [x] 2.1 Add `--initial-version <SEMVER>` arg to `Args` struct with `conflicts_with` for `release_version`, `release_from_pr`, and `changelog_entry`
- [x] 2.2 `get_all_commits()` not needed — revert of `get_commits_since_tag(None)` provides equivalent; safety moves to callers

## 3. Core: Validation logic

- [x] 3.1 Add validation that errors when a tag is reachable from HEAD and `--initial-version` is set
- [x] 3.2 Add validation that errors when `--initial-version` is not set, no tag exists, and we're in a write mode (not `--bump-type`)

## 4. Core: --bump-type path

- [x] 4.1 Update `run_bump_type()` to check for `--initial-version` when `get_last_tag()` returns None, scan all commits, and output the bump type

## 5. Core: Normal bump path

- [x] 5.1 Update `run()` to check for `--initial-version` when `get_last_tag()` returns None, scan all commits, compute the new version from `initial_version.bump(bump_type)`, and proceed with the normal bump pipeline (file updates, tag creation)

## 6. Tests

- [x] 6.1 Write integration test: `test_initial_version_bump_type_patch` — `--initial-version` with fix commits returns `patch`
- [x] 6.2 Write integration test: `test_initial_version_bump_type_minor` — `--initial-version` with feat commits returns `minor`
- [x] 6.3 Write integration test: `test_initial_version_normal_bump_creates_tag` — normal bump flow creates the first tag
- [x] 6.4 Write integration test: `test_initial_version_errors_when_tag_exists` — errors with clear message when a tag is present
- [x] 6.5 Write integration test: `test_initial_version_with_raw` — `--raw` outputs the bumped version
- [x] 6.6 Write integration test: `test_initial_version_invalid_semver` — invalid semver format errors
- [x] 6.7 Write integration test: `test_initial_version_no_tag_no_flag_errors_in_write_mode` — no tag without flag errors with suggestion

## 7. Verification

- [x] 7.1 Run full test suite: `cargo test` — all 65 tests pass
- [x] 7.2 Verify all existing tests still pass
