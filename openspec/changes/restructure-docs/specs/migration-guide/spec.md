# migration-guide Specification

## Purpose
Defines the content of `docs/migration-v4.md` — migration guide for users moving from v4 to v5.

## Requirements

### Requirement: documents v5.0.0 CLI contract changes
The file SHALL document the v5.0.0 breaking changes: exit code contract change (exit 0 for success including no-bump), `--raw` honoring `--preset`, new `--output json` flag, and release checksum requirements.

### Requirement: documents v5.2.0 workflow changes
The file SHALL document the v5.2.0 workflow restructuring: switch to release-please flow, tag-on-merge-commit pattern, and the new `--release-from-pr` flag.

### Requirement: documents scripting migration
The file SHALL document how to migrate v4 scripts that relied on exit codes to use `--bump-type` instead.

### Requirement: documents staying on v4
The file SHALL document how to stay on v4 if the v5 contract changes are unacceptable, including pinning to `@v4` or `@v4.9.4`.
