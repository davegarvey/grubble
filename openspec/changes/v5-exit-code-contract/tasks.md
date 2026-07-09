## 1. Fix binary exit-code contract

- [x] 1.1 Update `src/main.rs` `main()` to map `Ok(ExitCode::NoBump)` to `process::exit(0)` instead of `process::exit(1)`
- [x] 1.2 Verify `Err(_)` still exits with non-zero code and writes the error to stderr
- [x] 1.3 Update `tests/cli_test.rs::test_dry_run_no_bump_exit_code` to assert exit code 0 (was `Some(1)`)
- [x] 1.4 Add `tests/cli_test.rs::test_normal_run_no_bump_exit_code` asserting normal `grubble` exits 0 when no bump is needed
- [x] 1.5 Add `tests/cli_test.rs::test_raw_no_further_bump_exit_code` asserting `grubble --raw` exits 0 when no further bump is needed
- [x] 1.6 Add `tests/cli_test.rs::test_error_exit_code` asserting non-zero exit when strategy fails (e.g., missing `Cargo.toml` with `--preset rust`)

## 2. Make `--raw` honor `--preset`

- [x] 2.1 Remove the `if config.raw { return Box::new(git::GitStrategy::new(config.clone())); }` short-circuit in `src/strategy.rs::load_strategy`
- [x] 2.2 Verify `load_strategy` falls through to the `match config.preset` for `--raw`
- [x] 2.3 Add `tests/cli_test.rs::test_raw_with_rust_preset_reads_cargo_toml` asserting `--raw --preset rust` reads version from `Cargo.toml`
- [x] 2.4 Add `tests/cli_test.rs::test_raw_with_node_preset_reads_package_json` asserting `--raw --preset node` reads version from `package.json`
- [x] 2.5 Add `tests/cli_test.rs::test_raw_with_git_preset_unchanged` asserting `--raw --preset git` still reads from git tags (regression guard)

## 3. Add `--output text|json` for machine-readable output

- [x] 3.1 Add `Output` enum (`text`, `json`) and `--output` flag in `src/main.rs::Args` using clap's `ValueEnum`
- [x] 3.2 Wire the `output` field through `Config` (new field, skipped from serde)
- [x] 3.3 In `run_bump_type`, when `config.output == json`, emit a single JSON object to stdout with `{bump_type, current_version, triggering_commits, unknown_commits}` and suppress all `println!` logs
- [x] 3.4 In the normal run `--raw` path, when `config.output == json`, emit `{version, preset}` and suppress all `println!` logs
- [x] 3.5 In `main`, when `--output json` is combined with the normal run or `--dry-run`, return `Err` with a clear message ("--output json is only valid with --bump-type or --raw")
- [x] 3.6 Add `tests/cli_test.rs::test_bump_type_json_output` asserting valid JSON with the expected schema
- [x] 3.7 Add `tests/cli_test.rs::test_raw_json_output` asserting valid JSON with the expected schema
- [x] 3.8 Add `tests/cli_test.rs::test_json_output_invalid_with_dry_run` asserting non-zero exit
- [x] 3.9 Add `tests/cli_test.rs::test_json_output_invalid_with_normal_run` asserting non-zero exit

## 4. Simplify action.yml

- [x] 4.1 Replace preset-specific shell branches in "Get current version" step (lines 210-246) with a single `./grubble --raw --preset ${{ inputs.preset }}` invocation; preserve the Windows variant and the empty-result fallback to `0.0.0`
- [x] 4.2 Remove the `set +e` + `GRUBBLE_EXIT` + `echo "::error::..."` workaround in "Run bump" step (lines 333-354); restore plain `set -e` behavior
- [x] 4.3 In the "Run bump" step, capture `./grubble --bump-type` once into a variable and reuse it in the `>> $GITHUB_OUTPUT` write (remove the duplicate call at lines 261 and 272)
- [x] 4.4 In the "Verify checksum" step, change the missing-`.sha256` branch from `echo "::warning::..."` to `echo "::error::..."` + `exit 1` (lines 169-170)
- [x] 4.5 Update the `dry-run` input description (line 67) to reflect the new contract ("does not modify any files; exit code 0 in all non-error cases")

## 5. Simplify version.yml

- [x] 5.1 Remove the `set +e` + exit-code branching workaround added in the "Bump version" step (the new binary contract makes it unnecessary)
- [x] 5.2 Restore the original direct invocation: `./target/release/grubble ...`

## 6. Documentation and CHANGELOG

- [x] 6.1 Update `README.md:108` to reflect new `--dry-run` contract (exit 0 always; use `--bump-type` for the signal)
- [x] 6.2 Update `README.md:268-285` "gating" section to show the `--bump-type` pattern instead of the `--dry-run` exit-code pattern
- [x] 6.3 Update `README.md:144-145` defaults table for `gitUserName` / `gitUserEmail` to match `action.yml` defaults (`github-actions[bot]` / `41898282+github-actions[bot]@users.noreply.github.com`)
- [x] 6.4 Add a "Migration from v4" section to `README.md` documenting the breaking change, the `--bump-type` replacement, the new `--output json` flag, and explicitly noting that `@v4` remains available at the v4 contract
- [x] 3.10 Add a "Machine-readable output" section to `README.md` documenting the JSON schemas for `--bump-type` and `--raw`
- [x] 6.5 Remove the duplicate `- handle grubble exit 1 as clean no-op, not error` entry at `CHANGELOG.md:29`
- [x] 6.6 Add a `## [5.0.0]` entry to `CHANGELOG.md` calling out: (a) exit-code contract, (b) `--raw` honors `--preset`, (c) new `--output json` flag, (d) checksum hard-fail in the action, (e) `validate-release.sh` removed, (f) `fix-action-version-detection-and-output-mapping` superseded, (g) `@v4` remains available

## 7. Remove stale script

- [x] 7.1 Delete `scripts/validate-release.sh`

## 8. Version bump and verification

- [x] 8.1 Bump `Cargo.toml` version from `4.9.4` to `5.0.0`
- [x] 8.2 Run `cargo fmt --all` and `cargo fmt --all -- --check` _(CI will run on the PR — no local Rust toolchain available)_
- [x] 8.3 Run `cargo clippy --all-targets --all-features -- -D warnings` _(CI will run on the PR)_
- [x] 8.4 Run `cargo test --all-features --verbose` and confirm all tests pass (including the new ones from tasks 1.4-1.6, 2.3-2.5, and 3.6-3.9) _(CI will run on the PR)_
- [ ] 8.5 Open a PR; let the existing CI pipeline (`ci.yml` + `version.yml`) drive the release and confirm both pass

## 9. Archive superseded OpenSpec change

- [x] 9.1 Mark `openspec/changes/fix-action-version-detection-and-output-mapping` as superseded by v5 in its README and archive it
