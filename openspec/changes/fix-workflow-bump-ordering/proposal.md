## Why

The Version & Release workflow creates a redundant version bump (v5.6.0) after every release PR auto-merge. The root cause is step ordering: `Bump (dry-run)` runs before `Release merged PR`, so the dry-run never sees the tag just created by the Release step. It re-analyzes already-released commits and produces a stale next-version.

A secondary issue: the macOS build runner had a transient disk I/O failure (`unable to sync download to disk: Input/output error`). No retry logic exists to self-heal.

## What Changes

- Reorder workflow steps: `Detect` → `Release` → `Bump` → `Open` (currently `Detect` → `Bump` → `Release` → `Open`)
- The `Bump` step now runs `git fetch origin --tags --force` before the dry-run, so tags created by the `Release` step (via GitHub API) are visible to `git describe --tags`
- Add retry logic to `Build (native)` step (3 attempts with 15s delay)
- Add fallback `Setup Rust (retry)` step that runs if the main setup action fails
- Update the `canonical-release-workflow` spec to document the new ordering

## Capabilities

### New Capabilities

None — this is a structural refinement of the existing workflow.

### Modified Capabilities

- `canonical-release-workflow`: Step ordering requirements change from `Bump → Release → Open` to `Release → Bump → Open`. The "independent steps" concept is replaced with explicit ordering. The fetch-tags requirement moves from the `Open` step to the `Bump` step (defence-in-depth still in `Open`).

## Impact

- `.github/workflows/version.yml`: Steps reordered, Bump step gains `git fetch`, Open step loses stale-tag concern, build steps gain retry
- `openspec/specs/canonical-release-workflow/spec.md`: Updated requirements for step ordering and tag fetch
