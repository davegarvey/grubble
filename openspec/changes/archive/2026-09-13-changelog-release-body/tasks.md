## 1. Grubble CLI: --changelog-entry flag

- [x] 1.1 Add `read_latest_changelog_entry()` function to `src/changelog.rs` with `extract_changelog_entry()` parser
- [x] 1.2 Add unit tests for `extract_changelog_entry()`: multiple entries, single entry, empty file, no entries, header-only
- [x] 1.3 Add `--changelog-entry` flag to `Args` struct in `src/main.rs`
- [x] 1.4 Wire the flag in `run()`: call `changelog::read_latest_changelog_entry()` and print the result
- [x] 1.5 Ensure `--changelog-entry` is mutually exclusive with bump/commit modes (no side effects)
- [x] 1.6 Verify `cargo build` succeeds and `cargo test` passes

## 2. Workflow: version.yml restructure

- [x] 2.1 In `Open or update release PR` step: after `grubble` bump+changelog, run `grubble --changelog-entry` and use output as PR body via `gh pr create --body`
- [x] 2.2 Remove `create-release` job entirely (bubble + Node.js setup)
- [x] 2.3 Change `build-release` to `needs: [version, test]` and update checkout ref to `needs.version.outputs.tag_name`
- [x] 2.4 Change `publish-crate` to `needs: [test, build-release]` and update checkout ref to `needs.version.outputs.tag_name`
- [x] 2.5 Verify the `Release merged PR` step already uses `BODY` from `--release-from-pr` (which reads the PR body — now the changelog entry)

## 3. Documentation

- [x] 3.1 Update README CLI usage section to include `--changelog-entry`
- [x] 3.2 Update README to remove references to bubble/AI release notes and the `create-release` job
- [x] 3.3 Update release workflow documentation sections that describe the create-release job and bubble step
