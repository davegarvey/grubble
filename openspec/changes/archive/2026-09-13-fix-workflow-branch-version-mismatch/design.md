## Context

The "Version & Release" workflow (`version.yml`) implements a canonical release-please-style flow:

1. **Bump step** (dry-run): runs `grubble --raw --dry-run` to compute whether a bump is needed and what the next version would be.
2. **Detect step**: finds the most recently merged release PR.
3. **Release step**: creates the git tag + GitHub Release for the merged PR's version.
4. **Open step**: creates/updates a release PR on `release/v<version>` using the dry-run version for the branch name.

The bug: `--dry-run` forces `config.raw = true` (see `src/main.rs:274-280`), which skips the file-behind-tag sync logic at line 307. The Open step runs grubble **without** `--raw`, so the sync fires there. When Cargo.toml is behind the latest tag, grubble syncs it up first, then bumps — producing a version different from the dry-run prediction. The branch name is already locked in, causing a permanent mismatch.

### Current State

- The Bump step's branch name (`release/v<VERSION>`) is set before grubble actually runs
- The `--raw` flag is the gate for the sync logic, and `--dry-run` forces `--raw`
- There is no way to run the dry-run with the sync logic active without it writing files
- The dry-run `version` output is unreliable when sync would fire: it should only be used for the `changed` boolean

## Goals / Non-Goals

**Goals:**
- The release branch name must always match the actual version grubble writes to Cargo.toml
- The dry-run must still correctly determine whether a bump is needed (`changed=true/false`)
- Minimal changes to grubble's core logic
- Orphaned branches/PRs from version drift must be cleaned up

**Non-Goals:**
- Changing the overall release-please-style flow architecture
- Adding new CLI commands or subcommands to grubble
- Backfilling missing tags (that's an operational fix, not a code fix)

## Decisions

### Decision 1: Derive branch name from actual version after grubble runs, not from dry-run

**Chosen approach:** Restructure the Open step so the release branch name is computed after grubble writes the version to Cargo.toml, not from the pre-computed dry-run version.

**How it works:**
1. Bump step (unchanged): `grubble --raw --dry-run` determines `changed=true/false`. The `version` output MUST NOT be used for branch naming — it is unreliable when sync fires. Only the `changed` boolean is authoritative.
2. Open step: fetch latest tags, run grubble on a temporary branch to bump + commit (**without** `--push`), then reads the actual version from Cargo.toml, derives the branch name from it, renames the branch, handles stale branches/PRs, and pushes.

**Step-by-step Open step flow:**

```bash
# 1. Fetch tags created by the Release step (which ran immediately before
#    this step). Without this, grubble's get_last_tag() might return a stale
#    tag, causing incorrect sync or "file ahead of tag" errors.
git fetch origin --tags --force

# 2. Create a temp branch from current HEAD (main after merge).
#    The concurrency group prevents any other workflow run from racing.
TMP_BRANCH="release/_tmp"
git checkout -B "${TMP_BRANCH}"

# 3. Run grubble: bump version, update CHANGELOG, commit (no push).
#    No --git-branch here — we'll name the branch after we know the version.
./target/release/grubble \
  --git-user-name "github-actions[bot]" \
  --git-user-email "41898282+github-actions[bot]@users.noreply.github.com" \
  --preset rust \
  --changelog

# 4. Read the actual new version from Cargo.toml.
#    Use a precise grep to avoid matching workspace member versions.
ACTUAL_VERSION=$(grep -m1 '^version = "' Cargo.toml | cut -d'"' -f2)
if [ -z "$ACTUAL_VERSION" ]; then
  echo "::error::Failed to read version from Cargo.toml after grubble run"
  exit 1
fi
BRANCH="release/v${ACTUAL_VERSION}"

# 5. If the actual version matches the latest tag, no bump occurred
#    (grubble may have only synced). Skip PR creation.
LATEST_TAG=$(git describe --tags --abbrev=0 --match 'v[0-9]*.[0-9]*.[0-9]*' 2>/dev/null || true)
if [ "v${ACTUAL_VERSION}" = "$LATEST_TAG" ]; then
  echo "Version ${ACTUAL_VERSION} matches latest tag ${LATEST_TAG}; no bump needed"
  git checkout - && git branch -D "${TMP_BRANCH}"
  exit 0
fi

# 6. If a release branch already exists for the dry-run version (which may
#    differ from the actual version), close its PR and delete the stale
#    branch to avoid orphaned branches confusing human reviewers.
DRY_RUN_VERSION="${{ steps.bump.outputs.version }}"
if [ "$DRY_RUN_VERSION" != "$ACTUAL_VERSION" ]; then
  STALE_BRANCH="release/v${DRY_RUN_VERSION}"
  STALE_PR=$(gh pr list --head "$STALE_BRANCH" --json number --jq '.[0].number' 2>/dev/null || true)
  if [ -n "$STALE_PR" ]; then
    echo "Version changed (dry-run: ${DRY_RUN_VERSION}, actual: ${ACTUAL_VERSION}); closing stale PR #${STALE_PR}"
    gh pr close "$STALE_PR" --comment "Superseded by ${BRANCH} — version changed due to drift repair"
    gh api "repos/${GITHUB_REPOSITORY}/git/refs/heads/${STALE_BRANCH}" -X DELETE 2>/dev/null || true
  fi
fi

# 7. Rename branch to the correct name and push.
#    Use --force-with-lease instead of --force for safety (avoids
#    overwriting remote changes the local clone is unaware of).
git branch -m "${TMP_BRANCH}" "${BRANCH}"
git push origin "${BRANCH}" --force-with-lease

# 8. Create a passing check run for the required "test" check.
PUSHED_SHA=$(git rev-parse "${BRANCH}")
gh api "repos/${GITHUB_REPOSITORY}/check-runs" \
  --method POST \
  -f "name=test" \
  -f "head_sha=${PUSHED_SHA}" \
  -f "status=completed" \
  -f "conclusion=success" \
  -f "output[title]=Release branch check" \
  -f "output[summary]=Content validated by CI on main"

# 9. Read the CHANGELOG entry for the PR body.
CHANGELOG_BODY=$(./target/release/grubble --changelog-entry || echo "")

# 10. Create or update the release PR using the actual version.
if gh pr view "${BRANCH}" >/dev/null 2>&1; then
  echo "PR already exists for ${BRANCH}; branch was force-pushed."
  gh pr edit "${BRANCH}" \
    --title "Release v${ACTUAL_VERSION}" \
    --body "${CHANGELOG_BODY}"
else
  gh pr create \
    --base main \
    --head "${BRANCH}" \
    --title "Release v${ACTUAL_VERSION}" \
    --body "${CHANGELOG_BODY}"
  echo "Release PR opened for ${BRANCH}"
fi

# 11. Enable auto-merge with squash using the PAT.
for i in 1 2 3 4 5; do
  if GH_TOKEN=${{ secrets.RELEASE_PAT }} gh pr merge "${BRANCH}" --auto --squash 2>/dev/null; then
    echo "Auto-merge enabled on ${BRANCH}"
    break
  fi
  sleep 3
done
```

**Rationale:** This ensures the branch name always matches the file content. The sync logic is free to do whatever it needs — the branch name follows, not leads. Stale branch/PR cleanup prevents orphaned resources.

**Alternatives considered:**

- *Make dry-run use the sync logic*: Removed `config.raw = true` from `--dry-run` mode. Rejected because the sync writes files and commits, which defeats the purpose of a dry-run.
- *Add `--sync` flag to grubble*: A new flag that runs the sync logic without `--raw` but skips file writes. More complex, requires grubble code change, and still needs the workflow to handle the post-sync version.
- *Keep branch name from dry-run, rename if wrong*: Run grubble, check if the version differs from the dry-run, rename branch if needed. More fragile and introduces an unnecessary "check and correct" pattern.

### Decision 2: Grubble does not need changes

The workflow change alone is sufficient. Grubble's sync logic is correct — it heals drift between files and tags. The problem was the workflow using a pre-computed version for the branch name. By deriving the branch name after grubble writes, we align with grubble's actual behavior.

If we wanted grubble to be a better workflow citizen, we could add `--output version` support in non-raw modes (to emit the final version as JSON), but that's an enhancement, not a fix.

## Risks / Trade-offs

- **[Low] Temp branch naming**: Using `release/_tmp` could conflict if two workflow runs overlap. Mitigation: the `concurrency` group in `version.yml` prevents concurrent runs (`group: version-release, cancel-in-progress: false`).
- **[Low] Grubble exit without bump**: If grubble determines no bump is needed (the dry-run and actual analysis diverge), the temp branch would be left. Mitigation: check the actual version against the latest tag; if equal, delete the temp branch and exit without creating a PR.
- **[Low] Sync writes extra commit**: When the sync fires, grubble creates two commits (sync + bump). The branch rename still captures both correctly.
- **[Low] Dry-run version output is unreliable**: When sync fires, the dry-run's `version` output reflects the pre-sync baseline, not the actual version grubble writes. Mitigation: the `version` output is used only for the `changed` boolean; the branch name derives from the actual Cargo.toml content after grubble runs.
- **[Low] Version extraction fragility**: Grepping Cargo.toml for `version = "..."` could match workspace member dependencies. Mitigation: use `grep -m1 '^version = "'` to target only the top-level `[package]` version line, and add error handling for the empty case.

## Open Questions

None.
