## Why

The `Resolve release version` step in `action.yml` uses `${{ github.action_ref }}` directly in a `run:` block, which GitHub explicitly warns against for composite actions. This causes the ref to evaluate to an empty or unexpected value, bypassing the pinned-version download and falling through to `releases/latest`, which may lack release assets (404 error).

## What Changes

- Pass `github.action_ref` through an `env:` block in the composite action so it resolves correctly
- Use `gh api` instead of unauthenticated `curl` to the GitHub API to avoid rate limiting
- Add fallback handling for floating major/minor tags (e.g., `v5`, `v5.3`)
- Verify the resolved release actually has assets before attempting download

## Capabilities

### New Capabilities
- `robust-release-resolution`: Reliable resolution of the pinned action version when consuming grubble as a composite action from another repository

### Modified Capabilities
- (none — implementation change only, no spec-level requirement changes)

## Impact

- `action.yml` only — no changes to Rust source code, test behavior, or CLI interface
- The GitHub Action download flow becomes more resilient to API failures and user misconfiguration
