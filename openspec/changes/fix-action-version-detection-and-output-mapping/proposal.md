## Why

The grubble GitHub Action's "Get current version" step uses `./grubble --raw` which always forces `GitStrategy` (reads from git tags) regardless of the `--preset` setting. When there are no git tags (first run), this returns `0.0.0` instead of the actual version from the package file (e.g., `0.1.0`). This mismatch causes incorrect `previous_version` output and a subsequent `GITHUB_OUTPUT` processing failure (`Invalid format`), which marks the composite action step as failed and blocks downstream workflow steps like workspace version sync. Additionally, the composite action's output mapping for `previous-version` references `steps.bump.outputs.previous_version` which is never set.

## What Changes

- **"Get current version" step**: Replace `./grubble --raw` with preset-aware version extraction — `node -p` for the `node` preset, `grep` from `Cargo.toml` for the `rust` preset, keep `./grubble --raw` for the `git` preset or no preset
- **"Run bump" step output**: Add `echo "previous_version=$PREV" >> $GITHUB_OUTPUT` so the action's declared `previous-version` output resolves correctly
- **No change to the `--raw` CLI flag itself** — it still forces GitStrategy (documented behavior); only the action's "Get current version" step stops depending on it

## Capabilities

### New Capabilities
- `preset-aware-version-detection`: Read the current version using the appropriate method for the active preset (node/rust/git) instead of always using `--raw`/GitStrategy

### Modified Capabilities

None — there are no existing specs in this project.

## Impact

- **action.yml**: Two changes — the "Get current version" step and the "Run bump" step
- **Downstream workflows** consuming grubble's `previous-version` output: the value will now correctly reflect the preset's version source
- **No breaking changes**: Existing behavior is preserved when git tags exist (the version from tags matches the version from package files in steady state)
