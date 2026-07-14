# release-workflow-docs Specification

## Purpose
Defines the content of `docs/release-workflow.md` — documentation for the release-please flow, direct-push flow, reference implementations, and bypass token usage.

## Requirements

### Requirement: documents the release-please flow
The file SHALL document the recommended release-please flow: why to use it, the four-step process (commit → release PR → human merge → tag on merge commit), and how `--release-from-pr` fits in.

### Requirement: documents the GitHub Actions integration
The file SHALL include complete example workflows for the release-please flow using the grubble Action, including PR creation, PR detection, and release spec resolution.

### Requirement: documents grubble's own reference workflow
The file SHALL describe the reference `version.yml` workflow and how it implements the release-please pattern in practice.

### Requirement: documents the direct-push alternative
The file SHALL document the direct-push style for unprotected branches, including both the Action and manual setup approaches.

### Requirement: documents bypass token usage
The file SHALL document the bypass token approach for protected branches when the release-please flow cannot be used, including security considerations and the recommendation to prefer the release-please flow.
