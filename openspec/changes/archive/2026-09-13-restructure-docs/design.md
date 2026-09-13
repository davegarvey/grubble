## Context

The README is a single ~740-line document covering installation, CLI reference, configuration, workflow guides, best practices, CI/CD patterns, troubleshooting, version migration, and more. This monolithic structure makes it hard for users to find specific information.

Release-please (the canonical reference) keeps its README to ~300 lines of essentials and maintains dedicated docs pages for CLI reference, troubleshooting, configuration, migration, and design.

This restructure follows the same pattern: trim README to a concise landing page, move detailed documentation to `docs/` files, and cross-link between them.

## Goals / Non-Goals

**Goals:**
- Trim README from ~740 to ~200 lines (what it is, quick start, installation, brief usage overview, links to deeper docs)
- Create `docs/` directory with focused pages for each major topic
- Preserve all existing content — nothing removed, just reorganized
- Cross-link between README and docs pages so users can drill down

**Non-Goals:**
- Changing any code, CLI behavior, or workflow logic
- Removing content — every word is preserved in the new docs structure
- Changing CONTRIBUTING.md (it's already a separate file)
- Creating automated docs generation or CI for docs validation

## Decisions

### Decision 1: Flat `docs/` directory vs. nested subdirectories

**Choice:** Flat `docs/` directory with one file per topic (e.g., `docs/cli.md`, `docs/troubleshooting.md`).

**Rationale:** Matches release-please's flat `docs/` layout. Nested subdirectories add navigation overhead with no benefit at this scale. If docs grow significantly in the future, we can reorganize.

### Decision 2: File organization

The following files are created from the current README sections:

| docs/ file | Source sections in current README |
|---|---|
| `docs/cli.md` | Usage (full flag table) |
| `docs/configuration.md` | Configuration (full table), Versioning Strategies, Changelog Generation, Commit Types, Major/Minor Tag Tracking |
| `docs/release-workflow.md` | Releasing on a Git Server, Direct-Push Style, Bypass Token, Reference implementation |
| `docs/best-practices.md` | Best Practices (Releases, CI Gating, Commits, Pinned Versions) |
| `docs/ci-cd-patterns.md` | CI/CD Patterns (skip bump, read bump type, read version, machine-readable output), How It Works |
| `docs/migration-v4.md` | Migration from v4 (v5.0.0 changes, v5.2.0 changes, scripting migration, staying on v4) |
| `docs/troubleshooting.md` | Troubleshooting |

### Decision 3: README structure after trimming

The trimmed README will have:
1. Title + badges
2. Why (2-3 sentences)
3. Quick Start (install + example)
4. Installation (short: pre-built, cargo, from source, Action — link to docs/ for details)
5. Usage (concise flag summary, link to docs/cli.md)
6. Configuration (short example, link to docs/configuration.md)
7. Quick links to each docs/ page
8. Contributing (link to CONTRIBUTING.md)
9. License
10. For AI Users

## Risks / Trade-offs

- **Broken links**: External references pointing to `#section` anchors in the README will break if the section moves to a docs/ file. → Mitigation: search for external references to grubble's README anchors before restructuring (e.g., from other projects, the Action marketplace listing, GitHub issues). Internal links within grubble's own files (like CONTRIBUTING.md linking back to README) will be updated.
- **Discovery**: Users who only read the README may miss content that moved to docs/. → Mitigation: the trimmed README includes a clear "Quick Reference" section with links to each docs/ page, so everything is one click away.
- **Stale README after this change**: Future contributors might add content back to the README instead of the appropriate docs/ page. → Mitigation: CONTRIBUTING.md will note the docs/ structure.
