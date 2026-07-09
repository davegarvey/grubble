## Context

The grubble GitHub Action (`action.yml`) has two bugs in how it detects and reports versions:

1. **"Get current version" step** (line 210): Runs `./grubble --raw` which always loads `GitStrategy` (reads from git tags) regardless of the `--preset` input. When no git tags exist (first run), GitStrategy returns `0.0.0` instead of the actual version from the package file (e.g., `0.1.0` from `package.json`). This feeds a wrong `previous_version` into the action outputs and downstream logic.

2. **Output mapping** (line 81): The action declares `previous-version: ${{ steps.bump.outputs.previous_version }}`, but the "Run bump" step never writes `previous_version` to its `$GITHUB_OUTPUT`. It's only written in the "Get current version" step (`steps.current`), so the composite action's output resolves to empty/null.

The downstream impact surfaced on `davegarvey/work-os`: the `release.yml` workflow ran grubble on first merge (no tags), grubble bumped to `1.0.0` and pushed, but the `GITHUB_OUTPUT` failure marked the composite action step as failed, preventing the post-bump workspace version sync from running.

## Goals / Non-Goals

**Goals:**
- "Get current version" step accurately reads the version from the correct source based on the `--preset` input (node/rust/git)
- Action `previous-version` output correctly resolves from the bump step
- Backward compatible — no behavioral change when git tags exist and `--raw` would have returned the same value
- No changes to grubble's Rust source code; only the composite action shell scripts

**Non-Goals:**
- Not changing the `--raw` CLI flag behavior (it still forces GitStrategy intentionally)
- Not adding new inputs or outputs to the action
- Not changing the version bump logic itself

## Decisions

### 1. Preset-aware version extraction instead of `./grubble --raw`

**Decision**: In the "Get current version" step, replace `./grubble --raw` with:
- `preset=node` → `node -p "require('./package.json').version"`
- `preset=rust` → `grep '^version' Cargo.toml | head -1 | cut -d'"' -f2`
- `preset=git` or no preset → keep `./grubble --raw`

**Rationale**: `--raw` forces GitStrategy, which is only correct for the `git` preset. For file-based presets, extracting from the package file directly is simpler and avoids the code-change requirement in the Rust CLI. The approach mirrors what grubble's own `version.yml` already does for the Rust preset.

**Alternatives considered**:
- Modify grubble's Rust CLI to accept `--preset` with `--raw` — more robust but changes Rust code, requires release cycle
- Always use `./grubble --raw` but with a better fallback — doesn't fix the root cause; `--raw` returns git-tag version, not file version

### 2. Add `previous_version` to bump step outputs

**Decision**: Add `echo "previous_version=$PREV" >> $GITHUB_OUTPUT` in the "Run bump" step after the version comparison block.

**Rationale**: The action declares `steps.bump.outputs.previous_version` as the source for the `previous-version` output, so the bump step must actually write it. The `$PREV` variable is already computed from `steps.current.outputs.previous_version`, so this is a one-line addition.

### 3. Keep the same output name

**Decision**: Keep the action output `previous-version` unchanged (it still references `steps.bump.outputs.previous_version`).

**Rationale**: Changing the reference to `steps.current.outputs.previous_version` would also work, but it's more consistent to have the data flow through the bump step like the other outputs. If the bump step is skipped (bump-type-only or dry-run modes), the workflow exits early with `exit 0` before writing outputs, so only the normal path matters.

## Risks / Trade-offs

- **Preset mismatch** (low): If a user passes `--preset node` but `package.json` doesn't exist, `node -p` will fail. The fallback `echo "0.0.0"` handles this gracefully, matching the current behavior.
- **No tags on subsequent runs** (low): The fix handles the first-run case; on subsequent runs git tags exist, and even if they didn't, the preset-based extraction is now correct.
- **Node.js not installed** (low): The `node preset` already requires Node.js (grubble reads `package.json`), so `node -p` is a safe dependency. Similarly, the `rust` preset requires Rust but the grep approach doesn't need `cargo` — just basic shell tools.
