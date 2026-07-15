# CI/CD Patterns

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

## How It Works

1. Sync package files to the latest tag (for `rust` / `node` presets).
2. Analyze commits since the last tag using conventional commit rules.
3. Pick the highest bump (`major` / `minor` / `patch` / `none`).
4. Update package files and `CHANGELOG.md` if configured.
5. Create the bump commit.
6. Create tags, including `v4` / `v4.1` floating tags if enabled.
7. Push to the remote if configured.
