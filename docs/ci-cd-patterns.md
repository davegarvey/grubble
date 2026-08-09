# CI/CD Patterns

## Bump and capture (no tag)

Push a bump commit without creating a tag, and capture the version for later use. Recommended for the pre-merge phase of a release workflow — the tag lands on the merge commit, not the release branch:

```bash
# Gate first: a no-op run emits no version, so skip it up front
if [ "$(grubble --bump-type --preset node --package-files package.json)" = "none" ]; then
  echo "No bump needed"
  exit 0
fi

# Read-only peek for the branch name (--raw makes no changes)
VERSION=$(grubble --raw --preset node --package-files package.json)

# Bump + commit + push the release branch. No --tag: the tag is created
# later on the merge commit. --git-branch pushes with --set-upstream,
# so it works on a fresh branch (plain --push requires an upstream).
grubble --push --git-branch "release/v$VERSION" --output json \
  --preset node --package-files package.json
```

Only files are written, committed, and pushed — no git tag is created. The bumped version is available as `$VERSION` (from `--raw`) and echoed via `--output json`, so the caller can use it for branch names (`release/v${VERSION}`), PR descriptions, or the later tag step. See [Best Practices](best-practices.md#releases) for why tagging on the merge commit matters.

## Skip the bump when nothing changed

`grubble --bump-type` prints `major`, `minor`, `patch`, or `none`. Use it as a gate:

```bash
if [ "$(grubble --bump-type --preset rust)" != "none" ]; then
  grubble --push --tag --preset rust
fi
```

`grubble --dry-run` always exits 0 (success, including no-op). For the "would a bump happen?" signal, use `--bump-type`.

## Read the bump type

`grubble --bump-type` prints `major`, `minor`, `patch`, or `none`. Use it to drive conditional logic:

```bash
case "$(grubble --bump-type)" in
  major) notify "breaking release" ;;
  minor) notify "feature release" ;;
  patch) : ;;                  # silent patch
  none)   exit 0 ;;
esac
```

## Read the new version

`grubble --raw` prints the version that would be released without making changes. It honors `--preset` (e.g. `--preset rust` reads from `Cargo.toml`; `--preset node` reads from `package.json`; `--preset git` reads from the latest tag). Pair it with `gh release create` or a deploy step:

```bash
NEW_VERSION=$(grubble --raw --preset rust)
gh release create "v$NEW_VERSION" --title "Release v$NEW_VERSION"
```

## Machine-readable output

Both `--bump-type` and `--raw` accept `--output json` for stable, machine-readable output:

```bash
grubble --bump-type --output json
# {
#   "bump_type": "minor",
#   "current_version": "1.2.3",
#   "triggering_commits": ["Minor: feat: add login"],
#   "unknown_commits": []
# }

grubble --raw --preset rust --output json
# {
#   "version": "1.2.3",
#   "preset": "rust"
# }
```

`--output json` also works in bump mode (with `--push`, `--changelog`, etc.), emitting `{"version": "x.y.z"}` after files are written and pushed. This is useful for workflows that need the actual version without grepping package files. Use `--output json` from CI scripts that need to parse the result instead of shell-substring matching.

## First Release

On a repository with no git tags, grubble scans all commits from the beginning and bumps from `0.0.0`:

```bash
grubble --bump-type --preset rust
# patch, minor, major, or none — based on all commits

grubble --tag --preset rust
# Creates the first tag. Subsequent runs work automatically.
```

Use `--initial-version` to start from a different baseline (e.g., `grubble --initial-version 1.0.0 --tag`).

Bootstrapping is a one-time setup — do it locally, then the action works as normal for subsequent releases.

## How It Works

1. Sync package files to the latest tag (for `rust` / `node` presets).
2. Analyze commits since the last tag using conventional commit rules.
3. Pick the highest bump (`major` / `minor` / `patch` / `none`).
4. Update package files and `CHANGELOG.md` if configured.
5. Create the bump commit.
6. Create tags, including `v4` / `v4.1` floating tags if enabled.
7. Push to the remote if configured.
