# ci-cd-patterns-docs Specification

## Purpose
Defines the content of `docs/ci-cd-patterns.md` — CI/CD usage patterns and machine-readable output documentation.

## Requirements

### Requirement: covers skip-bump gating
The file SHALL document how to use `--bump-type` as a gate to skip CI steps when no bump is needed.

### Requirement: covers reading the bump type
The file SHALL document how to read the bump type and use it in conditional logic.

### Requirement: covers reading the new version
The file SHALL document how to read the next version with `--raw` and use it with `gh release create`.

### Requirement: covers JSON output
The file SHALL document `--output json` usage with `--bump-type`, `--raw`, and `--release-from-pr`, with example JSON schemas.

### Requirement: covers the internal process
The file SHALL document how grubble processes a version bump: sync files, analyze commits, compute bump, update files, commit, tag, push.
