## Why

The release workflow depends on a Node.js AI notes tool (`npx github:davegarvey/bubble`) that requires an OpenAI API key and runs as a separate job (`create-release`). If this job fails — expired API key, npm network issue, or bubble itself errors — the entire release asset pipeline is blocked: no binary archives are uploaded and no crate is published. The canonical release-please pattern generates the release body from the CHANGELOG entry directly, with no external dependencies.

## What Changes

- Add `--changelog-entry` flag to grubble that reads CHANGELOG.md and prints the most recent entry
- Update the `Open or update release PR` workflow step to use the changelog entry as the PR body
- Remove the `create-release` job (bubble dependency) from `version.yml`
- Update `build-release` to depend on `version` + `test` directly instead of `create-release`
- Update `publish-crate` to depend on `test` + `build-release` instead of `create-release`
- Update README with the new flag and remove references to bubble

## Capabilities

### New Capabilities
- `changelog-entry-flag`: a `--changelog-entry` CLI flag that reads the latest entry from CHANGELOG.md and prints it to stdout

### Modified Capabilities
- `canonical-release-workflow`: the release PR body now contains the CHANGELOG entry instead of a placeholder; the `create-release` job and its bubble dependency are removed; downstream jobs depend on `version` + `test` instead of `create-release`

## Impact

- `src/changelog.rs` — new `read_latest_changelog_entry()` function with unit tests
- `src/main.rs` — new `--changelog-entry` CLI flag
- `.github/workflows/version.yml` — restructured: bubble dependency removed, dependency graph simplified, PR body uses changelog entry
- `README.md` — document `--changelog-entry`, update workflow references to remove bubble
- The `OPENAI_API_KEY` secret is no longer needed
