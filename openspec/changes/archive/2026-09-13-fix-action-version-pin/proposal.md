## Why

The GitHub Action always downloads the **latest** release binary regardless of which version the user pinned (e.g., `@v5.2.1`). If the latest release has no binary assets (like v5.2.3), the download 404s even when the pinned version has valid assets. Users cannot pin to a known-good version.

## What Changes

- Replace the "Get latest release" API call with version resolution that respects `github.action_ref`
- Add validation: use the pinned release tag directly when it matches a semver pattern; fall back to the API for floating tags (`@v5`) or branch refs
- Apply the same fix to the checksum download step
- Include graceful fallback with a warning when the pinned version cannot be resolved

## Capabilities

### New Capabilities
- `action-binary-download`: Version-aware download of the grubble binary in the GitHub Action. The action respects the user's pinned version, falling back to the latest release when a specific release tag cannot be determined.

### Modified Capabilities
- *(none — binary download behavior is not governed by an existing spec)*

## Impact

- `action.yml` — the binary download steps (Get latest release / Download binary / Verify checksum)
- README usage examples remain unchanged (they use `@v5` floating tag)
