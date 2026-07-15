## Context

Grubble uses `git describe --tags` to find the last tag ancestor of HEAD, then analyses commits since that tag (`tag..HEAD`). When no tag is reachable (fresh repo), there is no baseline range, so grubble returns `"none"` for `--bump-type` and refuses to bump.

The `--initial-version <SEMVER>` flag fills this gap by letting the user explicitly provide the "last released version" as a semver string. When set and no tag is found, grubble scans all commits and determines the bump from the provided baseline. This is a pure git-native approach — no GitHub API dependency, no heuristic guessing.

## Goals / Non-Goals

**Goals:**
- Allow `--bump-type` to return a meaningful result on first run when `--initial-version` is set
- Allow the normal bump flow to create the first tag when `--initial-version` is set
- Error with a clear message if a tag already exists (contradictory state)
- Error with a clear message if `--initial-version` is not set and no tag exists (user guidance)
- Work with all presets (rust, node, git)

**Non-Goals:**
- Auto-detecting the initial version — the user must provide it explicitly
- Replacing or modifying `get_last_tag()` for normal (non-first-run) operation
- GitHub API integration or external release tracking

## Decisions

### D1: Separate `get_all_commits()` function

Rather than reverting the `get_commits_since_tag(None)` → `vec![]` fix, add a dedicated `get_all_commits()` to `git.rs` that runs `git log --pretty=%s`. This keeps the safety fix (no accidental full scans) while providing an explicit opt-in path.

**Alternatives considered:**
- Revert the fix and add a gate in callers: riskier, easier to accidentally scan
- Inline `git log --pretty=%s` at each call site: more duplication

### D2: Conflicts only with `--release-version` and `--release-from-pr`

`--initial-version` is a modifier, not an action. It should compose with `--bump-type`, `--raw`, `--dry-run`, `--tag`, `--push`, etc. It conflicts with `--release-version` (both determine a version to write, different logic) and `--release-from-pr` (unrelated).

**Alternatives considered:**
- Conflict with `--raw`: rejected because `--initial-version --raw` is useful — it shows what version would be created without writing files
- Conflict with `--dry-run`: rejected for the same reason

### D3: `--initial-version` errors when a tag is reachable from HEAD

If a tag already exists, the user doesn't need `--initial-version` — grubble already has a valid baseline. Erroring with a clear message prevents silent misuse.

### D4: Initial version is the bump baseline, not the current version

The initial version serves as the "last tag" for semantic comparison (`initial.bump(bump_type) = new_version`). The current file version (from strategy) is used for file-vs-tag validation but does NOT affect the bump computation. This matches the user's mental model: "I've never released, my last release was X."

## Risks / Trade-offs

- **[Risk] User provides a wrong initial version** → They get a wrong first tag. Mitigation: no different from providing any wrong argument; user is expected to know their own project.
- **[Risk] No distinction between "first run" and "post-cleanup"** → With `--initial-version` explicit, this is not a risk — the user is consciously declaring first-run state.
- **[Trade-off] One-time friction** → User must pass `--initial-version` on first run. Future runs have tags and work automatically. Acceptable — explicit is safer than guessing.
- **[Risk] Large repos with many commits may be slow on first run** → Mitigation: this is a one-time cost. If it's a problem, users can create an initial tag manually instead.

## Open Questions

- Should `--initial-version` work without any preset? Yes — it's a pure semver value. For the `git` preset, the current version is 0.0.0 and the initial version provides the baseline.
- Should `--raw --initial-version X.Y.Z` output the initial version or the bumped version? The bumped version (new_version). `--raw` outputs "what would the next version be."
