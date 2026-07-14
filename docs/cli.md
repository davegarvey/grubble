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
grubble --output json        # emit machine-readable output (with --raw or --bump-type)
grubble --quiet              # suppress the commit list
grubble --git-branch release/v0.35.0  # push the bump to a release branch (works on protected branches)
grubble --git-branch release/v0.35.0 --force-push  # same, with --force-with-lease (for workflow-owned branches)
grubble --changelog-entry             # print the latest entry from CHANGELOG.md
grubble --release-from-pr 79          # resolve a merged release PR to a tag spec (for canonical release-please workflows)
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
| `--output` | enum | `text` | Output format: `text` or `json`. Valid with `--bump-type`, `--raw`, or `--release-from-pr`. |
| `--release-from-pr` | int | — | Resolve a merged release PR to a tag spec. Requires `GH_TOKEN` or `GITHUB_TOKEN`. |
| `--changelog-entry` | bool | `false` | Read the latest entry from `CHANGELOG.md` and print it. |

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

`--output json` is rejected when combined with the normal run mode or `--dry-run` (those modes always print human-readable text). Use `--output json` from CI scripts that need to parse the result instead of shell-substring matching.
