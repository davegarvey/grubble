## 1. Version resolution in action.yml

- [x] 1.1 Replace the "Get latest release" step with version resolution that reads `github.action_ref`, strips ref prefixes, and validates semver pattern
- [x] 1.2 Add API call to `repos/davegarvey/grubble/releases/tags/{version}` for validated semver refs
- [x] 1.3 Add fallback to `releases/latest` with `::warning::` when the ref is not a specific semver tag
- [x] 1.4 Ensure the resolved version is written to `$GITHUB_OUTPUT` as `version=`

## 2. Checksum download alignment

- [x] 2.1 Verify the checksum download step uses `steps.release.outputs.version` (same output as the binary)
- [x] 2.2 No changes needed — it already uses the same output variable, so aligning the resolution fixes both

## 3. Validation and edge cases

- [x] 3.1 Test with pin `@v5.2.1` — downloads v5.2.1 binary, not latest
- [x] 3.2 Test with pin `@v5` — falls back to latest with warning
- [x] 3.3 Test with pin `@main` — falls back to latest with warning
- [x] 3.4 Test when pinned release has no assets (e.g., v5.2.3) — clear error message, not a cryptic 404
