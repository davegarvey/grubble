## Why

The release workflow (`version.yml`) is stuck in a permanent file-vs-tag deadlock: every release PR bumps Cargo.toml to the *next* dev version (e.g., 5.4.1) but the tag is created from the branch name (e.g., v5.3.2). The file is always ahead of the tag. The Open step then fails with "package version X is ahead of latest tag vY" and no further releases can be made until a manual fix.

Root cause: the Open step runs `grubble` in forward-bump mode, which triggers the file-behind-tag sync logic and produces a version that diverges from the dry-run prediction. A zombie tag (v5.4.0 on the wrong commit) previously cascaded through the sync logic, amplifying the divergence.

## What Changes

### 1. `grubble --release-version` flag

Add a new `--release-version <VERSION>` flag to grubble that sets Cargo.toml (or package.json) to an *exact* version and optionally generates the CHANGELOG entry. This is NOT a forward bump — it writes precisely the version the caller specifies. This eliminates the sync-logic problem because there's no file-vs-tag comparison.

### 2. version.yml Open step uses `--release-version`

The Open step will call `grubble --release-version <dry_run_output> --changelog --output json` instead of `grubble --preset rust --changelog --output json`. This means:
- The version written matches the dry-run prediction exactly
- No sync logic can fire
- No file-ahead-of-tag error can occur
- The branch name matches the PR title matches the file version matches the eventual tag

### 3. version.yml spec updated

The canonical-release-workflow spec scenarios that document the divergence behavior (dry-run vs actual) will be updated to reflect that divergence no longer occurs.

## Capabilities

### New Capabilities
- `release-version-flag`: New `--release-version` flag on the grubble CLI. Writes an exact version to the package file, generates CHANGELOG entry if `--changelog` is set, and commits. No bump logic, no file-vs-tag sync, no tag creation.

### Modified Capabilities
- `canonical-release-workflow`: The Open step will use `--release-version` instead of forward-bump. The scenarios documenting dry-run/actual version divergence will be removed since divergence no longer happens.

## Impact

- `src/main.rs`: Add `--release-version` clap arg, implement handler
- `src/strategy/`: Each strategy needs a `write_version()` method (or reuse existing `update_files()`)
- `src/changelog.rs`: Needs a function to generate an entry for a specified version (not just the bumped version)
- `.github/workflows/version.yml`: Replace the forward-bump call with `--release-version`
- `openspec/specs/canonical-release-workflow/spec.md`: Remove divergence scenarios
