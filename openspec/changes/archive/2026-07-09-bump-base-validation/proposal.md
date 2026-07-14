## Why

When the package file's version is ahead of the latest git tag (a state that shouldn't exist in a healthy repo), grubble currently uses the file version as the base for the bump — silently producing a wrong next version. This was the root cause of the v5.0.0 → v6.0.0 incident in PR #56: the PR manually bumped `Cargo.toml` to `5.0.0`, and the `Version & Release` workflow computed `5.0.0 + major = 6.0.0` instead of `4.9.4 + major = 5.0.0`.

The "file ahead of tag" state is anomalous. Silently producing a wrong version is the worst possible response; the right response is to fail loudly with an actionable error so a human can align the file and tag.

## What Changes

- **BREAKING** — When the package file's version is strictly greater than the latest tag's version (and `preset` is not `git`), `grubble` exits with a non-zero code and a clear, actionable error message naming both the file version and the tag version.
- Existing behavior preserved for all other cases: file == tag (no-op bump base), file < tag (sync up — current behavior).
- New CONTRIBUTING.md note: "Don't manually bump the package file in a release PR. The `Version & Release` workflow derives the next version from the latest tag plus conventional commits. Manually bumping puts the file ahead of the tag and grubble will refuse to bump."

## Capabilities

### New Capabilities
- `bump-base-validation`: Defines the contract for which file/tag relationship is valid and what grubble does when it detects an invalid relationship. Establishes that `grubble` MUST fail (not silently use either value) when the file version is strictly ahead of the tag version.

### Modified Capabilities
- None. The existing `cli-exit-codes` capability (added in v5.0.0) already covers the exit-code behavior for errors; this change just adds a new error condition to enforce.

## Impact

- **Binary users** with a manually-bumped file ahead of the tag: scripts will now exit non-zero with a clear error instead of producing a wrong version. This is a safer failure mode but a behavior change.
- **`action.yml` and `version.yml`**: unchanged. The new error will surface as a failed step with the error visible in CI logs.
- **`CONTRIBUTING.md`**: new section "Releasing" explaining the constraint.
- **Tests**: new test asserting the failure path.
- **No version-bump coupling**: the change is itself a patch bump, but the release will go through the same `Version & Release` workflow — which means **the v5.0.1 release PR must NOT manually bump `Cargo.toml`** (it would trigger the new error). The fix is purely in source files; the version bump is performed by the workflow.
