## Context

After the v5.0.0 release, a follow-up incident surfaced a latent bug in grubble: when the package file's version is ahead of the latest tag, grubble uses the file version as the base for the bump. In PR #56, `Cargo.toml` was manually bumped to `5.0.0`; the workflow then computed `5.0.0 + major = 6.0.0`, producing a wrong release.

The "file ahead of tag" state shouldn't exist in a healthy repo. The current code's sync step only handles "file behind tag" (syncs up). There's no handling for the symmetric case, and silently producing a wrong version is the worst response.

This change makes grubble fail loudly when it detects the inconsistent state, with an actionable error message. The accompanying CONTRIBUTING note explains why and how to avoid triggering the error in the first place.

## Goals / Non-Goals

**Goals:**
- Make the "file ahead of tag" state impossible to ignore: grubble fails with a clear, actionable error
- Preserve the existing "file behind tag" sync behavior
- Document the constraint in CONTRIBUTING.md so contributors don't trigger the error
- Ship as v5.0.1 (patch bump)

**Non-Goals:**
- Changing the bump base calculation in any other way (e.g., always using the tag regardless of file)
- Adding a `--force` or `--allow-ahead-of-tag` flag (deliberate friction; the user must align the state, not bypass it)
- Refactoring the sync step into a separate function
- Changing the v5.0.0 contract for any other case

## Decisions

### 1. Fail (don't silently use the tag or the file)

**Decision:** When `current_version > tag_ver` (and `preset != "git"`), return `Err(BumperError::InvalidConfig(...))` with both values named and both fix paths stated.

**Why over silently using the tag:** Silently using the tag is a "fix" that masks the anomaly. If the inconsistency had a real cause (e.g., a partial workflow failure leaving the file ahead of the tag), masking it means the next workflow run will silently do the same thing, and the state can drift further. A loud failure is the right default for a versioning tool.

**Why over silently using the file (current behavior):** This is the bug. Producing a wrong version is worse than failing.

**Why over adding a `--force` flag:** Tempting, but adds complexity for an edge case. If a user genuinely needs to "use the file as base" they can do so by aligning the tag (creating a `v5.0.0` tag), which is the same end state.

### 2. Error message format

```
Error: Invalid configuration: package version 5.0.0 (in Cargo.toml) is 
ahead of latest tag 4.9.4. Refusing to bump.

To fix, align the file and tag. Either:
  - Revert Cargo.toml to match the latest tag, or
  - Create the missing tag: git tag v5.0.0 && git push origin v5.0.0
```

The message names both values, states what was attempted, and gives two concrete fix paths. The error variant is `BumperError::InvalidConfig`, which already exists (added in v5.0.0) and is rendered as `Error: Invalid configuration: <message>` to stderr.

### 3. Skip the check when `preset == "git"`

The `git` preset doesn't read from a file (the `GitStrategy::get_current_version()` reads from the latest tag), so the file-vs-tag comparison is meaningless. The check only applies to `rust` and `node` presets.

### 4. Skip the check in read-only modes

`--raw` and `--dry-run` are read-only and must not fail just because the state is anomalous — they're used to inspect the state. The check only fires during a real bump run.

### 5. CONTRIBUTING.md note, not README.md

The audience for this is contributors (people opening release PRs or working on grubble itself), not end users. CONTRIBUTING.md is the right home. The note explains:
- Why the file should not be manually bumped
- What the workflow does instead
- The error message they'll see if they do

### 6. Patch bump, not major

This is a behavior change but a safer one (failing where we were silently wrong). It's not a breaking change in the same sense as v5.0.0 — existing scripts that don't trigger the "file ahead of tag" state see no difference. So `5.0.0 → 5.0.1`.

**Self-referential wrinkle:** The v5.0.1 PR must NOT manually bump `Cargo.toml` (that would trigger the new error in its own release). The PR contains only source + test + doc changes; the `Version & Release` workflow performs the bump from the latest tag (`v5.0.0`).

## Risks / Trade-offs

- **[Risk] A user has a legitimate use case for "file ahead of tag"** (e.g., a manual version bump before tagging for compliance reasons) → Mitigation: the error message tells them exactly how to satisfy the constraint (create the tag). Their workflow is one extra command; not a blocker.
- **[Risk] The check fires in the v5.0.1 PR's own release** → Mitigation: the PR contains no `Cargo.toml` change. The workflow sees commits since v5.0.0 (the PR merge commit only), analyzes the `fix:` prefix → patch bump → v5.0.1. The PR's commits won't be in scope during the v5.0.1 release run; the v5.0.1 bump itself happens *after* the PR merges.
- **[Risk] Users upgrade and hit a new error** → Mitigation: the error message is actionable and references both the file and tag versions. The CONTRIBUTING note documents the pattern.
- **[Trade-off] Slightly more friction in CI** than silently using the file. Acceptable because the silent path was wrong.

## Migration Plan

1. Land this PR with the source + test + CONTRIBUTING changes.
2. The `Version & Release` workflow on main detects the `fix:` commit, runs grubble: file == tag (both `5.0.0`), no sync, no failure (the new check doesn't fire because file == tag), bump to `5.0.1`.
3. Release published. Existing v5.0.0 users see no change unless they hit the "file ahead of tag" state, in which case they get a clear error and a fix path.

**Rollback:** Revert the merge commit. v5.0.0 was correct; this only adds a stricter precondition.

## Open Questions

- _None._ The design is constrained: there's one new failure mode, one error message, one doc note. Decisions are settled.
