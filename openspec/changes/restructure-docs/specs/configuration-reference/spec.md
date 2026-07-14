# configuration-reference Specification

## Purpose
Defines the content of `docs/configuration.md` — complete configuration documentation for grubble.

## Requirements

### Requirement: docs/configuration.md covers all config options
The file SHALL document every option in `.versionrc.json` with its CLI flag equivalent, default value, and description.

### Requirement: covers presets and versioning strategies
The file SHALL document the `git`, `rust`, and `node` presets, what files each preset reads/updates, and how switching presets works.

### Requirement: covers changelog generation
The file SHALL document the changelog format, commit-to-category mapping (feat→Added, fix→Fixed, etc.), and markdown compliance.

### Requirement: covers commit type mapping
The file SHALL document the default commit type → bump type mapping and how to override it via the `types` option.

### Requirement: covers tag tracking
The file SHALL document `--update-major-tag` and `--update-minor-tag` behavior, including force-push semantics.
