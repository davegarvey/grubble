# best-practices-docs Specification

## Purpose
Defines the content of `docs/best-practices.md` — best practices for using grubble and designing release workflows.

## Requirements

### Requirement: covers release best practices
The file SHALL document release best practices including: preferring the release-please flow, auto-merge pitfalls on protected branches, Open/Release step independence, tagging strategy, floating tag semantics, force-push with `--force-with-lease`, and the `release-from-pr` Action input.

### Requirement: covers CI gating best practices
The file SHALL document CI gating best practices including: using `--raw --dry-run` or `--bump-type` instead of exit codes, using `--output json` for machine-readable output, and pinning strategies.

### Requirement: covers commit best practices
The file SHALL document commit best practices including: conventional commit format, breaking change markers (`!` or `BREAKING CHANGE:`), not editing release PRs directly, and squash-merging PRs.

### Requirement: covers version pinning best practices
The file SHALL document version pinning best practices including: using specific tags for release workflows, using major tags for public contracts, and the implications of moving a major tag.
