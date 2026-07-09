## Why

`grubble` currently exits 1 for both "no version bump needed" and "command failed." Every caller (`action.yml`, `version.yml`, user scripts) has to special-case exit 1 as a clean no-op, which has led to repeated workarounds shipped across 4.9.0–4.9.4. The v5 release fixes the root cause: any successful run — including a clean no-op — exits 0; only genuine errors exit non-zero. Bundled in: hard enforcement of release checksums in the GitHub Action, and unification of `--raw` so it honors `--preset` (collapsing the preset-specific shell branches in `action.yml` and resolving the in-flight OpenSpec change `fix-action-version-detection-and-output-mapping`).

## What Changes

- **BREAKING** — Normal `grubble` run exits 0 when no bump is needed (currently exit 1)
- **BREAKING** — `grubble --raw` always exits 0 on success; honors `--preset` for non-git presets (was always git-only; exit 1 on no further bump)
- **BREAKING** — `grubble --dry-run` exits 0 on success; exit code no longer signals "bump needed." Callers use `grubble --bump-type` for that signal
- GitHub Action requires `.sha256` checksum files for releases (currently warn-and-continue)
- GitHub Action's "Get current version" step uses `./grubble --raw --preset <preset>` (was preset-specific shell branches)
- Add `--output text|json` flag for `--bump-type` and `--raw` modes, providing machine-readable output for CI integrations
- Remove stale `scripts/validate-release.sh` (refers to non-existent `release.yml` and old `bumper` binary)
- Fix duplicate entry in `CHANGELOG.md` (4.9.2)
- Fix wrong defaults in `README.md` for `gitUserName` / `gitUserEmail` (currently shows `grubble-bot` / `grubble-bot@noreply.local`; action defaults to `github-actions[bot]`)
- Fix double `./grubble --bump-type` invocation in `action.yml` (lines 261, 272)

## Capabilities

### New Capabilities
- `cli-exit-codes`: Defines the exit-code contract for all CLI invocations of `grubble`, including the normal run, `--dry-run`, `--bump-type`, and `--raw` modes. Establishes that any successful outcome (including clean no-op) exits 0; only errors exit non-zero.
- `raw-mode-preset-awareness`: Defines how `--raw` interacts with `--preset` and what output is produced. Establishes that `--raw` honors `--preset` for non-git presets and exits 0 whenever a version is successfully produced.
- `machine-readable-output`: Defines the `--output text|json` flag for `--bump-type` and `--raw` modes, providing stable JSON schemas for CI integrations and downstream tooling.
- `release-checksum-verification`: Defines checksum enforcement behavior in the GitHub Action. Establishes that a missing `.sha256` for a release asset is a hard failure (was warning).

### Modified Capabilities
_None — `openspec/specs/` is empty, so no existing spec requirements to modify._

## Impact

- **Binary users** upgrading v4 → v5 must update scripts that use `--dry-run` as an exit-code gate; switch to `--bump-type` for the signal.
- **`action.yml`** simplifies: removes the preset-specific branches in "Get current version," removes the `set +e` + exit-code branching around the bump step, and removes the double `--bump-type` invocation.
- **`version.yml`** simplifies: removes the workaround I just added in the "Bump version" step.
- **`README.md`** updates: exit-code documentation for `--dry-run` and `--raw`; corrected defaults for `gitUserName` / `gitUserEmail`; JSON output examples.
- **`tests/cli_test.rs`** updates: `test_dry_run_no_bump_exit_code` (line 160) asserts the new contract; new tests for JSON output.
- **Resolves** the in-flight OpenSpec change `fix-action-version-detection-and-output-mapping` by fixing `--raw` in the binary rather than patching the action shell.
- **`@v4` floating tag** remains at `v4.9.4` and is frozen at the v4 contract. Users who want the v4 behavior can pin `@v4`, `@v4.9.4`, or any `@v4.x.x` indefinitely. The default floating tag shifts to `@v5` once v5 ships.
- **Crates.io / GitHub release** version bump from 4.9.4 → 5.0.0.
