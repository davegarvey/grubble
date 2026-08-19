# Configuration

Grubble reads `.versionrc.json` from the project root. Flags and file values are merged, with flags winning. Git tags are the default version source.

## Options

| Option | CLI flag | Default | Description |
| --- | --- | --- | --- |
| `preset` | `--preset` | `git` | Versioning strategy: `git`, `rust`, `node`, or `python`. |
| `packageFiles` | `--package-files` | `[]` | JSON array of files to update (for `rust` / `node` / `python`). The CLI flag accepts a comma-separated list. |
| `tagPrefix` | `--tag-prefix` | `v` | Prefix for git tags. |
| `commitPrefix` | `--commit-prefix` | `chore: bump version` | Prefix for the bump commit message. |
| `tag` | `--tag` | `false` | Create a git tag for the new version. |
| `push` | `--push` | `false` | Push the commit (and tag) to the remote. |
| `releaseNotes` | `--release-notes` (`-r`) | `false` | Include release notes in the tag annotation. Requires `tag: true`. |
| `changelog` | `--changelog` | `false` | Generate or update `CHANGELOG.md`. |
| `updateMajorTag` | `--update-major-tag` | `false` | Maintain a floating `v4` tag pointing to the latest `v4.x.x`. |
| `updateMinorTag` | `--update-minor-tag` | `false` | Maintain a floating `v4.1` tag pointing to the latest `v4.1.x`. |
| `gitUserName` | `--git-user-name` | `github-actions[bot]` | Identity used for the bump commit when no local git user is configured. |
| `gitUserEmail` | `--git-user-email` | `41898282+github-actions[bot]@users.noreply.github.com` | Email used for the bump commit when no local git user is configured. |
| `types` | — | see [Commit Types](#commit-types) | Per-type bump behavior. Valid values: `major`, `minor`, `patch`, `none`. |

`packageFiles` uses a JSON array in `.versionrc.json`:

```json
{
  "preset": "python",
  "packageFiles": ["pyproject.toml", "agentflow/main.py"]
}
```

The equivalent CLI input is `--package-files pyproject.toml,agentflow/main.py`.

If your repo has a local `user.name` / `user.email` set, grubble uses those and ignores `gitUserName` / `gitUserEmail`. In CI, set these to match your bot user (e.g. `github-actions[bot]`).

## Versioning Strategies

The `preset` option controls what files grubble writes.

- **`git`** (default and fallback) — tracks versions via `vX.Y.Z` tags only. No files are modified. Use this for monorepos, projects with their own versioning scheme, and languages that do not have a built-in Grubble preset. Pass the tag-derived version to the project's build or packaging script when an artifact needs an embedded version.
- **`rust`** — updates the `version` field in `Cargo.toml` and refreshes `Cargo.lock`. Pairs with `cargo publish`.
- **`node`** — updates the `version` field in `package.json` and `package-lock.json`. Pairs with `npm publish`.
- **`python`** — updates the PEP 621 `version` field in `pyproject.toml` (`[project]` / `[tool.poetry]`) and `__version__` / `VERSION` constant lines in listed package files. Pairs with `pip publish` / `twine upload`.

For a project without a supported package manifest, omit `preset` or set it explicitly to `git`:

```json
{
  "preset": "git",
  "changelog": true
}
```

`changelog` is optional for direct tag releases. Enable it for the release-PR flow so the release branch contains a tracked change even when Grubble does not update a package file. Keep `push`, `tag`, and floating-tag options out of shared configuration unless every invocation is intended to perform those actions; set them in the workflow instead.

When switching from `git` to a file-based preset, or when a package file is behind the latest tag, grubble first syncs the file to the tag (with a `chore: sync package version to v...` commit) and then proceeds with the normal bump.

## Changelog Generation

`grubble --changelog` writes a `CHANGELOG.md` in [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format:

- `feat:` → **Added**
- `fix:` → **Fixed**
- `perf:`, `refactor:` → **Changed**
- `revert:` → **Removed**
- `security:` → **Security**
- Breaking changes → **Changed** with a **BREAKING:** prefix

The changelog is committed as part of the release.

## Commit Types

Default mappings (override with the `types` option in `.versionrc.json`):

| Commit | Bump |
| --- | --- |
| `!` or `BREAKING CHANGE` | major |
| `feat:` | minor |
| `fix:` | patch |
| `perf:`, `refactor:` | none by default; usually remapped to `patch` or `minor` |
| `docs:`, `test:`, `chore:`, `ci:`, `build:`, `style:` | none |

Custom mapping example:

```json
{
  "types": {
    "perf": "minor",
    "revert": "none"
  }
}
```

## Major / Minor Tag Tracking

When `updateMajorTag` is enabled, grubble maintains a lightweight tag that follows the latest release in its range:

- `v4` → latest `v4.x.x`
- `v4.1` → latest `v4.1.x` (additionally requires `updateMinorTag`)

This is the convention GitHub Actions use, so consumers can reference `uses: owner/repo@v4` and automatically get the latest v4 release.

```bash
grubble --tag --push --update-major-tag --update-minor-tag
```

These tags are force-pushed. Anyone who has them checked out locally will need to re-fetch. Only enable this if you follow semver strictly — a breaking change bumps the major and consumers pinned to the old `v4` will pick it up.
