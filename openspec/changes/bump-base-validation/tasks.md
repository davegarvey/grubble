## 1. Implement the bump-base check

- [x] 1.1 In `src/main.rs::run`, inside the `if !config.raw { ... if let Some(tag_ver) = last_tag_version { ... } ... }` block, add a check: when `config.preset != "git"` and `current_version > tag_ver`, return `Err(BumperError::InvalidConfig(format!("package version {} (in {}) is ahead of latest tag {}. Refusing to bump. ...", ...)))`. The error message must name both the file version and the tag version and reference both fix paths ("revert" the file, or "create" the missing tag).
- [x] 1.2 Place the check BEFORE the existing "sync if file behind tag" branch so the failure is the first thing reported, not an after-thought.
- [x] 1.3 Verify the check is inside the `!config.raw` block so `--raw` and `--dry-run` still work even when the file is ahead of the tag.

## 2. Add tests

- [x] 2.1 Add `tests/cli_test.rs::test_file_ahead_of_tag_fails` asserting non-zero exit and that stderr contains the file version (`5.0.0`) and the tag version (`4.9.4`) and a fix-path keyword (`revert` or `tag`).
- [x] 2.2 Add `tests/cli_test.rs::test_file_behind_tag_syncs` (regression guard) asserting the existing sync-up behavior still works.
- [x] 2.3 Add `tests/cli_test.rs::test_file_ahead_of_tag_succeeds_in_raw_mode` asserting that `--raw --preset rust` exits 0 even when the file is ahead of the tag (read-only modes skip the check).
- [x] 2.4 Run `cargo test --all-features --verbose` and confirm all tests pass (including the new ones and all v5.0.0 tests).

## 3. Document the constraint

- [x] 3.1 Add a "Releasing" section to `CONTRIBUTING.md` (or update the existing release-process section) explaining: (a) do not manually bump `Cargo.toml` or `package.json` in a release PR, (b) the `Version & Release` workflow derives the next version from the latest tag plus conventional commits, (c) manually bumping the file puts it ahead of the tag and grubble will refuse to bump, with a clear error message.

## 4. Verify

- [x] 4.1 Run `cargo fmt --all` and `cargo fmt --all -- --check`.
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and confirm clean.
- [x] 4.3 Confirm the OpenSpec change validates: `openspec validate bump-base-validation`.
- [ ] 4.4 Open a PR; let `ci.yml` and `version.yml` drive the release. The PR's commits use the `fix:` prefix so the workflow bumps to v5.0.1 automatically (no manual `Cargo.toml` bump in the PR).

## 5. Post-merge verification

- [ ] 5.1 After merge, confirm the v5.0.1 release is published with correct assets.
- [ ] 5.2 Confirm the `v5` floating tag is still pointing to the latest v5.x.x release (v5.0.1).
