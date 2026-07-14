## 1. Grubble CLI: --changelog-entry flag

- [ ] 1.1 Add `read_latest_changelog_entry()` function to `src/changelog.rs` with `extract_changelog_entry()` parser
- [ ] 1.2 Add unit tests for `extract_changelog_entry()`: multiple entries, single entry, empty file, no entries, header-only
- [ ] 1.3 Add `--changelog-entry` flag to `Args` struct in `src/main.rs`
- [ ] 1.4 Wire the flag in `run()`: call `changelog::read_latest_changelog_entry()` and print the result
- [ ] 1.5 Ensure `--changelog-entry` is mutually exclusive with bump/commit modes (no side effects)
- [ ] 1.6 Verify `cargo build` succeeds and `cargo test` passes

## 2. Workflow: version.yml restructure

- [ ] 2.1 In `Open or update release PR` step: after `grubble` bump+changelog, run `grubble --changelog-entry` and use output as PR body via `gh pr create --body`
- [ ] 2.2 Remove `create-release` job entirely (bubble + Node.js setup)
- [ ] 2.3 Change `build-release` to `needs: [version, test]` and update checkout ref to `needs.version.outputs.tag_name`
- [ ] 2.4 Change `publish-crate` to `needs: [test, build-release]` and update checkout ref to `needs.version.outputs.tag_name`
- [ ] 2.5 Verify the `Release merged PR` step already uses `BODY` from `--release-from-pr` (which reads the PR body — now the changelog entry)

## 3. Documentation

- [ ] 3.1 Update README CLI usage section to include `--changelog-entry`
- [ ] 3.2 Update README to remove references to bubble/AI release notes and the `create-release` job
- [ ] 3.3 Update release workflow documentation sections that describe the create-release job and bubble step
