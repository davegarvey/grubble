## Context

The `action.yml` composite action has a "Resolve release version" step that determines which release binary to download. It reads `${{ github.action_ref }}` directly in a `run:` block, which GitHub's documentation explicitly warns not to do for composite actions. The fix needs to resolve the correct pinned tag and handle fallbacks robustly.

## Goals / Non-Goals

**Goals:**
- `github.action_ref` resolves correctly inside the composite action
- Floating tags (`v5`, `v5.3`) resolve to the latest matching full semver release
- GitHub API calls are authenticated to avoid rate limiting
- Resolution validates the target release actually has downloadable assets
- The action uses `@v5` (floating major tag) to always get the latest v5.x release with assets

**Non-Goals:**
- No changes to the Rust binary, CLI flags, or test behavior
- No changes to the version.yml workflow (self-hosted build path is unaffected)

## Decisions

1. **`env:` block for `github.action_ref`** — Pass the context through `env:` instead of using it directly in `run:`. This is the documented workaround for composite actions. The environment variable `$ACTION_REF` will reliably contain the ref value.

2. **`gh api` over `curl`** — Use `gh api` (already available on GitHub-hosted runners) instead of unauthenticated `curl`. This inherits the runner's `GITHUB_TOKEN` auth, avoiding the 60 req/hr unauthenticated rate limit. No new dependency.

3. **Resolve floating tags via API** — When `github.action_ref` returns `v5` or `v5.3` (valid GitHub release tags but not full semver), query the API to find the latest release matching that major/minor prefix rather than falling through to `releases/latest`. Use `gh release list --exclude-drafts --exclude-pre-releases --limit 50` and filter.

4. **Validate assets exist** — After resolving the version, check that the release has at least one binary asset before downloading. If not, emit an actionable error message.

## Risks / Trade-offs

- `gh api` depends on `GITHUB_TOKEN` being set in the runner environment. GitHub-hosted runners always have this, but self-hosted runners may not. The step should fail gracefully with a clear message if the token is missing.
- Floating tag resolution (`v5` → `v5.3.0`) adds API calls and complexity. Trade-off: users pinning to `@v5` get predictable behavior instead of silently tracking `releases/latest`.
