## Why

The workflow currently greps `Cargo.toml` to discover what version grubble actually wrote after a bump. This is fragile (matches any `version = "..."` line) and roundabout — grubble wrote the file, it should be the source of truth for what version it wrote. Adding `--output json` support to the bump path lets grubble emit the version as structured JSON, and the workflow can parse it with `jq`. This follows the release-please pattern where tools emit machine-parseable output for CI/CD consumption.

## What Changes

- **`grubble --output json` in bump mode**: Remove the restriction that limits `--output json` to `--raw`, `--bump-type`, or `--release-from-pr`. After the bump path (file writes, commit, push), emit `{"version": "x.y.z"}` to stdout.
- **Clean stdout for parsing**: When `--output json` is active, route informational log messages to stderr so stdout contains only the JSON payload.
- **Workflow version extraction**: Replace `grep -m1 '^version = "' Cargo.toml | cut -d'"' -f2` with parsing the JSON output via `jq -r '.version'`.

## Capabilities

### New Capabilities
- _None_

### Modified Capabilities
- `canonical-release-workflow`: The "Open step derives branch name from actual version written" requirement currently specifies grepping Cargo.toml for version extraction. This changes to parsing JSON output from grubble.

## Impact

- `src/main.rs`: Remove `--output json` restriction; emit JSON after bump path; route `log()` to stderr when JSON output requested.
- `.github/workflows/version.yml`: Replace grep-based version extraction with JSON parsing.
