## 1. Fix `github.action_ref` resolution in composite action

- [x] 1.1 Move `${{ github.action_ref }}` access to an `env:` block in the "Resolve release version" step
- [x] 1.2 Read the ref from the environment variable (`$ACTION_REF`) instead of inline expression

## 2. Replace unauthenticated curl with authenticated gh api

- [x] 2.1 Replace `curl -sS "$RELEASE_URL"` with `gh api` call for release-by-tag lookup
- [x] 2.2 Replace `curl -sS "$LATEST_URL"` with `gh api` call for latest release fallback

## 3. Handle floating major/minor tags

- [x] 3.1 Add regex match for floating major tag (`^v[0-9]+$`) — query releases API to find latest release matching the major version prefix
- [x] 3.2 Add regex match for floating minor tag (`^v[0-9]+\.[0-9]+$`) — query releases API to find latest release matching the major.minor prefix
- [x] 3.3 Ensure `@v5` resolves to latest v5.x release with assets (not `releases/latest`)

## 4. Validate release has assets before download

- [x] 4.1 After resolving the release version, check `assets` array length from API response
- [x] 4.2 If assets are empty, emit a clear error message with the release tag and instructions
- [x] 4.3 If assets are present, proceed with download

## 5. Graceful handling of missing GITHUB_TOKEN

- [x] 5.1 Detect if `GITHUB_TOKEN` is available; if missing, fall back to unauthenticated API access
- [x] 5.2 Emit a warning when falling back to unauthenticated mode

## 6. Verify and test

- [x] 6.1 Review final `action.yml` changes for shell quoting and edge cases
- [x] 6.2 Run `action.yml` validation (no formal test, but verify syntax and logic)
