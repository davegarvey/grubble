## Context

`grubble` v4.9.4 has an exit-code contract that conflates "no version bump needed" with "command failed" — both exit 1. This has forced every caller to special-case exit 1 as a clean no-op, leading to repeated workarounds in `action.yml` (`set +e` + exit-code branching), `version.yml` (the same pattern, added in 4.9.2), and a documented "gate" pattern in `README.md`. The contract is also internally inconsistent: `grubble --bump-type` always exits 0, but `grubble --dry-run` uses exit 1 to signal "no bump."

The `--raw` flag has a separate, related defect: `src/strategy.rs:14-17` short-circuits to `GitStrategy` whenever `config.raw` is true, ignoring `--preset`. The in-flight OpenSpec change `fix-action-version-detection-and-output-mapping` documents the downstream impact and works around it in the action shell.

v5.0.0 fixes both at the source rather than continuing to paper over them in shell scripts. The action's checksum enforcement is also tightened because v4.9.4 silently warns (and proceeds) when a release is missing a `.sha256` — a security gap that v5 closes.

The breaking-change surface is contained: anyone scripting directly against `grubble` with `--dry-run` or `--raw` as exit-code gates must migrate. The GitHub Action and `version.yml` are internal consumers and can be simplified once the binary contract is fixed.

## Goals / Non-Goals

**Goals:**
- Establish a POSIX-clean exit-code contract: success (including no-op) is 0; only errors are non-zero
- Make `--raw` honor `--preset` so the action no longer needs preset-specific shell branches
- Hard-fail the action on missing or mismatched release checksums
- Resolve the in-flight OpenSpec change `fix-action-version-detection-and-output-mapping` by fixing the binary, not the action shell
- Bundle small, low-risk cleanups (`validate-release.sh` removal, CHANGELOG dedup, README default correction, action.yml double `--bump-type` call)

**Non-Goals:**
- Adding a `--json` output mode (deferred to v5.x)
- Changing the meaning of `--bump-type` output
- Renaming or removing any existing flags
- Changing the version string format or preset behavior beyond the `--raw` short-circuit
- Bumping dependencies (handled in a separate v5.0.x)

## Decisions

### 1. Two-state exit code (0 = success, non-zero = error)

`src/main.rs:422-431` currently maps `Ok(NoBump)` and `Err(_)` to exit 1. Change `Ok(NoBump)` to also exit 0. Errors continue to exit 1 (the generic, unhandled case) with stderr output.

**Why over a three-state contract (0=ok, 1=no-bump, 2+=errors):** A three-state contract still breaks `set -e` (it treats any non-zero as failure) and complicates `if cmd; then` (caller must handle two "success" codes). POSIX tooling expects success = 0. Callers who need to distinguish "did anything change" use `--bump-type`, which already prints `major|minor|patch|none` and exits 0.

**Alternatives considered:**
- *Three-state (0/1/2)*: rejected — breaks `set -e`, complicates conditional flow.
- *Always exit 0, signal via stdout only*: rejected — errors would be invisible to `set -e` and CI status checks.

### 2. `--raw` honors `--preset` by removing the strategy short-circuit

`src/strategy.rs:14-17` returns `GitStrategy` whenever `config.raw` is true. Remove the `if config.raw` branch and let the existing `match config.preset` handle it.

**Why:** The short-circuit is undocumented behavior that contradicts the spec described in the action's `preset` input and the `runs.using` / `--preset` flag. Removing it makes `--raw` a pure "give me the current/next version" mode that respects all other config.

**Migration impact for direct binary users:** None for the `git` preset (unchanged). For `node`/`rust` presets, `--raw` now reads from the package file rather than the git tag — which is what every existing user wants (it's why the in-flight OpenSpec change was opened). No back-compat flag is added: the prior behavior was a bug, not a feature.

### 3. Checksum enforcement becomes a hard failure in the action

`action.yml:163-189`'s "Verify checksum" step currently prints `::warning::` and continues when `.sha256` is missing. Change to `::error::` and `exit 1`. Checksum mismatches already exit 1; this change just adds the missing-checksum case to the same hard-fail path.

**Why:** `version.yml:230-240` generates `.sha256` for every release asset. A release without one indicates an upstream problem that should fail loudly. The `0.0.0` fallback used by the existing test for missing artifacts (`action.yml:381-382`) is unrelated and stays.

**Migration impact:** None for normal use — every v5 release will have checksums. Operators who upgrade to a `@v5` pin will benefit immediately; operators who stay on `@v4` (floating) are unaffected because the action.yml at v4 tags retains the v4 behavior.

### 4. Cleanup items bundled in v5.0.0

- `scripts/validate-release.sh`: refers to `release.yml` (file is `version.yml`), `target/release/bump` (binary is `grubble`), and outdated actions. Has been broken since the `bumper` → `grubble` rename. Delete.
- `CHANGELOG.md:28-29`: duplicate `- handle grubble exit 1 as clean no-op, not error` entry under 4.9.2. Remove the duplicate.
- `README.md:144-145`: `gitUserName` / `gitUserEmail` table shows `grubble-bot` / `grubble-bot@noreply.local`; the action defaults in `action.yml:49-53` are `github-actions[bot]` / `41898282+github-actions[bot]@users.noreply.github.com`. Update the table.
- `action.yml:261, 272`: the bump-type and dry-run short-circuit paths each call `./grubble --bump-type` twice. Capture once and reuse.

These are not spec changes; they're mechanical fixes shipped alongside the major version bump. Each is under 10 LOC.

### 5. Version bump 4.9.4 → 5.0.0

Per semver, the exit-code contract change is a breaking change for any caller that relies on `grubble` exit codes. `--raw`'s preset-awareness change is a behavioral change for non-git presets (the prior behavior was a bug, but it was observable behavior, so a major bump is the safe call).

### 6. Add `--output text|json` for machine-readable CI integration

Add a new `--output` flag (clap `ValueEnum`) accepting `text` (default) and `json`. Applies only to `--bump-type` and `--raw` modes — the two modes that produce structured information that CI scripts frequently parse.

**Schemas:**

`--bump-type --output json`:
```json
{
  "bump_type": "minor",
  "current_version": "1.2.3",
  "triggering_commits": ["Minor: feat: add new endpoint"],
  "unknown_commits": []
}
```

`--raw --output json`:
```json
{
  "version": "1.2.4",
  "preset": "rust"
}
```

`--output` is rejected with a non-zero exit when combined with the normal run mode or `--dry-run` (those modes always print human-readable text and changing their output format would break existing scripts). The clap `value_enum` makes the error message clean.

**Why:** Repeatedly, CI scripts use shell substring matching or `awk` to parse `grubble --bump-type` or `grubble --raw` output. A stable JSON schema eliminates a class of brittle parsers and is a small, contained addition that costs ~40 LOC.

**Why now (in v5):** v5 is the breaking-change release; adding a new flag is non-breaking. Users adopting v5 will benefit immediately. The schema is small and stable; lock it in for v5.x.

**Why over separate `--json` flag:** `--output` is the established convention (`gh`, `cargo`, `rustc`, `kubectl` all use it) and naturally extends to other formats later (e.g., `--output yaml`).

**Why not add `--json` to the normal run too:** The normal run's stdout is mixed informational text and file output. Forcing JSON there would require restructuring the entire output pipeline. Out of scope.

### 7. `@v4` floating tag is frozen, not deleted

`version.yml`'s release process creates a new major-version tag (`v5`) when v5.0.0 ships. The `v4` floating tag is not touched — it remains at `v4.9.4`. All `@v4.x.x` specific tags remain reachable. Users can pin `@v4`, `@v4.9.4`, etc. indefinitely and continue to get the v4 contract.

The `v4` floating tag stops *advancing* (no more v4.x.x releases are cut) but it does not disappear. The default floating tag for new consumers shifts to `@v5` because that's what the v5 release announcement points to.

Maintenance policy for the v4 line (security backports vs. full freeze) is a project-level decision and is intentionally left out of scope here.

## Risks / Trade-offs

- **[Risk] External users with `if grubble --dry-run; then …` scripts will break silently** → Mitigation: prominent CHANGELOG entry, README "Migration from v4" section, and a clear callout in the v5.0.0 release notes.
- **[Risk] Users on `@v4` floating tag don't get the v5 contract** → Mitigation: this is intentional; the v4 floating tag is a stability commitment. Operators must move to `@v5` to get the new contract. Documented in CHANGELOG.
- **[Risk] `--raw` reading from a package file when no git tag exists could change output for some edge cases** (e.g., a `node` preset run on a repo with no `package.json`) → Mitigation: `RustStrategy` / `NodeStrategy` already error clearly on missing files; the existing `|| true` fallback in `action.yml:381-382` handles the empty-stdout case.
- **[Risk] Checksum hard-fail could block users on rare missing-checksum releases** → Mitigation: every v5+ release will have checksums. The only exposure is to operators who pin `@v5` and somehow fetch an asset from a non-standard location — out of scope.
- **[Trade-off] v5 has fewer lines of code in `action.yml` and `version.yml` but more in `src/main.rs`/`src/strategy.rs`.** Net: clearer architecture; the shell-side workarounds are deleted entirely.

## Migration Plan

1. Land binary changes (`src/main.rs`, `src/strategy.rs`) on a `feat/v5` branch. Run `cargo test --all-features` and the existing CLI tests.
2. Update `action.yml` to use the new `--raw --preset <preset>` pattern; remove the workarounds and the duplicate `--bump-type` call.
3. Update `version.yml` to remove the `set +e` workaround (the binary's new contract makes it unnecessary).
4. Apply doc/CHANGELOG cleanups.
5. Delete `scripts/validate-release.sh`.
6. Bump `Cargo.toml` version 4.9.4 → 5.0.0.
7. PR with full diff; let the existing CI pipeline (`ci.yml` + `version.yml`) drive the release.
8. Release notes call out: (a) exit-code contract, (b) `--raw` reads from package file for non-git presets, (c) checksum enforcement, (d) the new `--output json` flag, (e) the @v4 → @v5 migration and the fact that `@v4` remains available.
9. Archive the in-flight OpenSpec change `fix-action-version-detection-and-output-mapping` as superseded.

**Rollback strategy:** v5 is a single commit series; revert the merge commit and re-tag v4.9.4 if a critical regression surfaces. Action consumers can pin back to `@v4.9.4` immediately.

## Open Questions

_Resolved during scoping:_
- *JSON output mode?* Yes, included in v5 as `--output text|json` for `--bump-type` and `--raw`. Schema is locked in the `machine-readable-output` spec.
- *`@v4` maintenance?* Frozen at `v4.9.4` and reachable indefinitely. No deletions. v4 contract is preserved by the action.yml at the v4 tag; it is not retroactively updated.
