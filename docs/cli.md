# CLI Reference

## Usage

```bash
grubble                      # bump based on commits since the last tag
grubble --push               # push the bump commit
grubble --tag                # create a git tag for the new version
grubble --changelog          # generate or update CHANGELOG.md
grubble --update-major-tag   # also maintain a floating v4 tag
grubble --raw                # print the new version, no changes
grubble --dry-run            # preview the bump without applying it
grubble --bump-type          # print major | minor | patch | none
grubble --output json        # emit machine-readable output (--raw, --bump-type, or bump path)
grubble --quiet              # suppress the commit list
grubble --git-branch release/v0.35.0  # push the bump to a release branch (works on protected branches)
grubble --git-branch release/v0.35.0 --force-push  # same, with --force-with-lease (for workflow-owned branches)
grubble --changelog-entry             # print the latest entry from CHANGELOG.md
grubble --release-from-pr 79          # resolve a merged release PR to a tag spec (for canonical release-please workflows)
grubble --initial-version 0.1.0       # override baseline version for first-release detection
grubble --help               # full flag reference
```

Every flag has a corresponding option in `.versionrc.json` (see [Configuration](configuration.md)). CLI flags override file values.

## Flag Reference

| Flag | Type | Default | Description |
| --- | --- | --- | --- |
| `--push` | bool | `false` | Push the commit (and tag) to the remote. |
| `--force-push` | bool | `false` | Force push with `--force-with-lease`. Requires `--git-branch`. |
| `--git-branch` | string | `""` | Push the bump to this branch instead of HEAD. |
| `--quiet` | bool | `false` | Suppress commit list output. |
| `--tag` | bool | `false` | Create a git tag for the new version. |
| `--release-notes` (`-r`) | bool | `false` | Include release notes in the tag annotation. Requires `--tag`. |
| `--raw` | bool | `false` | Print the new version string (dry run, no changes). |
| `--preset` | string | `git` | Versioning strategy: `git`, `rust`, or `node`. |
| `--tag-prefix` | string | `v` | Prefix for git tags. |
| `--commit-prefix` | string | `chore: bump version` | Prefix for the bump commit message. |
| `--package-files` | string | `""` | Comma-separated list of files to update. |
| `--git-user-name` | string | `github-actions[bot]` | Identity used for the bump commit. |
| `--git-user-email` | string | `41898282+github-actions[bot]@users.noreply.github.com` | Email used for the bump commit. |
| `--update-major-tag` | bool | `false` | Maintain a floating `v4` tag pointing to the latest `v4.x.x`. |
| `--update-minor-tag` | bool | `false` | Maintain a floating `v4.1` tag pointing to the latest `v4.1.x`. |
| `--changelog` | bool | `false` | Generate or update `CHANGELOG.md`. |
| `--bump-type` | bool | `false` | Print `major`, `minor`, `patch`, or `none` and exit. |
| `--dry-run` | bool | `false` | Check if bump is needed without making changes. |
| `--output` | enum | `text` | Output format: `text` or `json`. Valid in all modes. In bump mode emits `{"version": "x.y.z"}` after the commit/push. |
| `--release-from-pr` | int | — | Resolve a merged release PR to a tag spec. Requires `GH_TOKEN` or `GITHUB_TOKEN`. |
| `--changelog-entry` | bool | `false` | Read the latest entry from `CHANGELOG.md` and print it. |
| `--initial-version` | semver | — | Override the baseline version for first-release detection (default: `0.0.0`). When no git tag exists, grubble scans all commits and bumps from `0.0.0`. Use this to start from a different version. Incompatible with `--release-version`, `--release-from-pr`, and `--changelog-entry`. |

## `--release-from-pr` output

```json
{
  "version": "5.2.2",
  "tag_name": "v5.2.2",
  "major_tag_name": "v5",
  "merge_commit_sha": "582643110c4fa6e5279f9f6705fbbc1bf1e52143",
  "title": "Release v5.2.2",
  "body": "## [5.2.2] - 2026-07-14\n\n### Fixed\n\n- use merge commit so release tag stays reachable"
}
```

## JSON output examples

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

`--output json` works in all modes. In bump mode (with `--push`, `--changelog`, etc.) it emits `{"version": "x.y.z"}` after files are written and pushed. Use `--output json` from CI scripts that need to parse the result instead of shell-substring matching.

## First Release (Bootstrapping)

When a repository has no git tags, grubble scans all commits from the beginning and bumps from `0.0.0` by default:

```bash
# On a fresh repo with commits but no tags:
git init
git add .
git commit -m "feat: initial implementation"
git commit -m "fix: resolve edge case"

grubble --bump-type
# minor — detects the feat: commit

grubble --tag
# Creates tag v0.1.0 on the bump commit.
# Subsequent runs detect the tag and work normally.
```

Use `--initial-version` to override the baseline if your project should start from a different version:

```bash
grubble --initial-version 1.0.0 --tag
# Creates tag v1.0.1 (bumped from 1.0.0).
```

`--initial-version` errors if a tag already exists — it's only for repos with no previous tags.
