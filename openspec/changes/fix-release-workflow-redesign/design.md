## Context

The release workflow (`version.yml`) uses a forward-bump pattern in the Open step: it runs `grubble --preset rust --changelog --output json` which triggers the full bump pipeline — sync/validate → analysis → bump. This pipeline has two critical problems for the release-please flow:

1. **Sync logic can fire**: if the file is behind a tag (e.g., a zombie tag), the sync logic changes Cargo.toml to the tag version, then bumps forward from there. The actual version diverges from the dry-run prediction.

2. **File-ahead-of-tag error**: after a release, Cargo.toml contains the next dev version but the tag points to the release version. On the next run, the non-raw bump hits the guard and refuses to proceed.

Both problems stem from using a forward-bump tool when what's needed is a "set to exact version" operation.

## Goals / Non-Goals

**Goals:**
- The Open step writes exactly the version the dry-run predicted, every time
- No sync logic, no file-vs-tag comparison in the Open step
- The release branch name, PR title, file version, and eventual tag all match
- Backward compatible: existing workflows using forward-bump continue to work

**Non-Goals:**
- Changing the Release step (tag/release creation) — it works correctly
- Changing the dry-run step — it correctly computes the next version
- Changing the CI workflow
- Adding auto-recovery for stuck states (not needed once the flow is correct)

## Decisions

### Decision 1: New `--release-version` flag instead of modifying version.yml to use sed

**Chosen:** Add `--release-version <VERSION>` to grubble.
**Rejected:** Writing Cargo.toml directly with sed and generating CHANGELOG manually in the workflow script.

**Rationale:**
- The CHANGELOG generation uses the same commit analysis as the bump flow. Duplicating that logic in shell scripts would be fragile and hard to maintain.
- grubble already has `Strategy::update_files()` for writing versions to package files. Reusing it is clean.
- The CHANGELOG entry should include the same commits and formatting as a normal bump-generated entry. Using `changelog::generate_changelog_entry()` directly is consistent.
- A new flag makes the intent explicit: "set to this exact version, don't bump forward."

### Decision 2: `--release-version` reuses `get_commits_since_tag()` for CHANGELOG

**Chosen:** When `--changelog` is combined with `--release-version`, grubble gets commits since the last tag (same as the bump flow) and passes them to `generate_changelog_entry()`. The version in the CHANGELOG entry is the specified release version, not the bumped version.

**Rationale:** The CHANGELOG should list all commits since the last release. The last tag is the correct boundary. Using the same function as the bump flow ensures consistent commit selection.

### Decision 3: `--release-version` skips all sync/validate logic

**Chosen:** The handler runs before the main bump pipeline. It writes the version, generates CHANGELOG (if requested), and commits. No file-vs-tag comparison, no analysis, no sync.

**Rationale:** The caller (version.yml Open step) has already determined the correct version via dry-run. There's no need to re-validate or sync. The file-ahead-of-tag guard is explicitly bypassed because the caller is intentionally setting the version.

### Decision 4: `--release-version` uses git config from `.versionrc.json`

**Chosen:** The git user name/email configured in `.versionrc.json` (or via CLI flags) is used for the commit. The commit prefix is also respected.

**Rationale:** Consistency with the forward-bump flow. The same user identity and commit message format should be used.

## Risks / Trade-offs

- **[Low] Duplicate commits on version conflicts**: If the specified version matches the commits since the last tag AND the current file version, the commit created by `--release-version` duplicates what would have been created by a forward bump. This is harmless (idempotent).
- **[Low] CHANGELOG entry with wrong version**: The CHANGELOG entry is generated with the specified version, but the commits are fetched from `git log <last_tag>..HEAD`. If the workflow passes an incorrect version, the CHANGELOG entry will be incorrect. Mitigation: the version comes from the dry-run step which is correct.
- **[Low] --release-version with incompatible flags**: `--release-version` must conflict with `--raw`, `--dry-run`, `--bump-type`, and `--release-from-pr` since it's a write operation. Enforced via clap's `conflicts_with_all`.
