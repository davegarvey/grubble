## Why

The README has grown to ~740 lines covering installation, CLI reference, workflow guides, best practices, CI/CD patterns, troubleshooting, migration guides, and more. This makes it hard to find the information you need. Release-please keeps its README concise (~300 lines) by splitting detailed docs into a `docs/` directory with focused pages for troubleshooting, CLI reference, configuration, migration, etc.

## What Changes

- Create `docs/` directory with dedicated pages for each major topic
- Trim README.md to essentials: what it is, quick start, installation, brief usage overview, and links to docs/
- Move detailed sections to docs/ with cross-links
- No structural or behavior changes to any code or workflows

## Capabilities

### New Capabilities
- `cli-reference`: full CLI flag reference in `docs/cli.md`
- `configuration-reference`: complete configuration documentation in `docs/configuration.md`
- `release-workflow-docs`: release-please flow, direct-push flow, bypass token docs in `docs/release-workflow.md`
- `best-practices-docs`: best practices in `docs/best-practices.md`
- `ci-cd-patterns-docs`: CI/CD usage patterns in `docs/ci-cd-patterns.md`
- `migration-guide`: v4-to-v5 migration in `docs/migration-v4.md`
- `troubleshooting-guide`: troubleshooting in `docs/troubleshooting.md`

### Modified Capabilities
None — this is a documentation-only change. No spec-level requirements are altered.

## Impact

- `README.md` — trimmed to ~200 lines
- `docs/` — 7 new documentation files created
- No code, workflow, or configuration changes
