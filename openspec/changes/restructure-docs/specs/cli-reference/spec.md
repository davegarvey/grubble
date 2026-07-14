# cli-reference Specification

## Purpose
Defines the content and structure of `docs/cli.md` — the full CLI flag reference for grubble.

## Requirements

### Requirement: docs/cli.md lists every CLI flag
The file SHALL document every CLI flag supported by grubble, including: `--push`, `--force-push`, `--git-branch`, `--quiet`, `--tag`, `--release-notes`, `--raw`, `--preset`, `--tag-prefix`, `--commit-prefix`, `--package-files`, `--git-user-name`, `--git-user-email`, `--update-major-tag`, `--update-minor-tag`, `--changelog`, `--bump-type`, `--dry-run`, `--output`, `--release-from-pr`, `--changelog-entry`.

### Requirement: each flag includes type, default, and description
Each flag entry SHALL include its CLI argument name, value type, default value, and a description of what it does.

### Requirement: examples section
The file SHALL include usage examples for common flag combinations, including: dry-run preview, bump-type output, JSON output, changelog generation, release PR resolution, and release notes extraction.
