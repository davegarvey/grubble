## Context

The `.versionrc.json` file configures default behavior for grubble runs. With `"tag": true`, every grubble invocation creates git tags. In the release-please workflow, the Open step runs `grubble --changelog --push --force-push` — this loads the config file and attempts to create tags. Since the Release step (or a previous run) already created these tags via `gh api`, the tag push fails with `! [rejected] v5.2.4 -> v5.2.4 (already exists)`, causing the entire Open step to fail and the release PR to never be created.

## Goals / Non-Goals

**Goals:**
- Remove tag-related config options so the Open step's `grubble` invocation doesn't attempt to create tags
- Local `grubble` without `--tag` won't create tags — consistent with the canonical release-please pattern

**Non-Goals:**
- Changing the Release step's tag creation (it uses `gh api`, not grubble)
- Changing how grubble handles tags when `--tag` is explicitly passed

## Decisions

**Choice:** Remove `"tag"`, `"releaseNotes"`, and `"updateMajorTag"` from `.versionrc.json`.

**Rationale:** The canonical release-please workflow creates tags via the GitHub API in the Release merged PR step. The config file should reflect this canonical pattern. Users running `grubble` locally can pass `--tag --update-major-tag` explicitly when they want tags.
