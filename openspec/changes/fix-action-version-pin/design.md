## Context

The composite action in `action.yml` (lines 115–216) downloads the grubble binary at runtime. Currently it fetches `releases/latest` from the GitHub API to determine which version to download. This ignores the user's version pin (`uses: davegarvey/grubble@v5.2.1`), meaning:

- Pinning is purely cosmetic — the action code matches the pinned version but the binary doesn't
- If the latest release lacks assets (v5.2.3 has zero binary assets), downloads 404 regardless of pinning to a working version
- Users can't lock to a known-good version

## Goals / Non-Goals

**Goals:**
- Resolve the download version from `github.action_ref` when the ref is a specific semver tag (`vX.Y.Z`)
- Fall back to `releases/latest` for floating tags (`@v5`), branch refs (`@main`), or SHA refs
- Apply the same logic to both the binary download and checksum download URLs
- Print actionable warnings when falling back so users understand what's happening

**Non-Goals:**
- Resolving `@v5` to the latest `v5.x.x` release (stays as `releases/latest`)
- Adding retry logic or alternative download mirrors
- Removing the API call entirely (still needed for fallback)
- Changing how the binary is extracted or verified

## Decisions

### Decision 1: Query `releases/tags/{ref}` instead of mutating the API call URL

**Chosen:** Query the GitHub API endpoint `repos/davegarvey/grubble/releases/tags/{version}` to validate the ref is a real release before downloading. This gives us a clean error path: if the API returns 404, fall back to `releases/latest`.

**Alternatives considered:**
- *Use `github.action_ref` directly in the download URL* — Simpler, but a 404 from the download gives a cryptic error vs. a clear warning at the API level. Also, branch refs like `main` would produce a malformed URL (`releases/download/main/...`) instead of failing gracefully.
- *Strip `refs/tags/` and use the tag directly* — Doesn't handle the floating tag case and gives no validation.
- *Resolve the ref to a commit, then find the nearest release* — Over-engineered for this use case.

### Decision 2: Strip ref prefixes from `github.action_ref`

**Chosen:** Apply `${VERSION#refs/tags/}` and `${VERSION#refs/heads/}` to normalize the value. Some runner versions return `github.action_ref` with these prefixes (actions/runner#662, fixed but still present on older self-hosted runners).

### Decision 3: Validate semver pattern before using the ref

**Chosen:** Check `github.action_ref` matches `^v?[0-9]+\.[0-9]+\.[0-9]+$` before attempting the tag-specific API call. If it doesn't match (e.g., `v5`, `main`, `abc123`), skip straight to the API fallback with a warning.

**Rationale:** `v5` matches a git tag but is not a release tag. Validating the pattern avoids a wasted API round-trip and keeps the error path clean.

### Decision 4: Keep `GITHUB_OUTPUT` for the resolved version

**Chosen:** Continue writing `version=<resolved>` to `$GITHUB_OUTPUT` so downstream steps (download, checksum) work identically as before. The downstream steps don't change — only the resolution method changes.

## Risks / Trade-offs

- **[Risk] `github.action_ref` might not be set on very old runners** → Mitigation: the fallback to `releases/latest` preserves existing behavior, so old runners silently degrade rather than breaking.
- **[Risk] User pins `@v5.2.3` which has no assets** → The fix correctly downloads from v5.2.3, which 404s. This is *correct* behavior — the user pinned a broken release. The action should surface a clear error message: "Release v5.2.3 has no binary assets."
- **[Trade-off] Floating `@v5` still uses `releases/latest`** → This matches existing behavior. The fix only changes behavior when a specific version is pinned. Users of `@v5` get identical behavior to today.
- **[Risk] GitHub API rate limiting** → Two API calls in the fallback path (`releases/tags/{ref}` then `releases/latest`). Each action run uses 1-2 API calls, well within limits. No caching needed.

## Open Questions

- Should the action validate that the resolved release has assets (non-empty assets array from the API) before attempting the download, or let the download 404 naturally?
- Should `@v5` (floating major) be resolved to the latest `v5.x.x` by filtering `releases?per_page=100` for tags matching `^v5\.`, rather than using `releases/latest`? This would be a future enhancement.
