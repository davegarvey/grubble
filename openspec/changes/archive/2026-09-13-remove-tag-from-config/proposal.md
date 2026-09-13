## Why

`.versionrc.json` has `"tag": true` and `"updateMajorTag": true`, which causes every `grubble` invocation to create git tags. In the release-please workflow, the Open step runs `grubble` to bump files and push the release branch — but tag creation fails because tags already exist from a previous Release step. This blocks the release PR from being created.

## What Changes

- Remove `"tag"`, `"releaseNotes"`, and `"updateMajorTag"` from `.versionrc.json`
- These options are not needed because the release-please workflow creates tags post-merge via `gh api` in the Release merged PR step, not via `grubble`

## Capabilities

None — this is a configuration-only change with no new capabilities.

## Impact

- `.versionrc.json` — removed three config keys
- Local `grubble` runs without `--tag` will no longer create tags (consistent with the canonical release-please pattern)
