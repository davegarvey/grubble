# Grubble

[![CI](https://github.com/davegarvey/grubble/actions/workflows/ci.yml/badge.svg)](https://github.com/davegarvey/grubble/actions/workflows/ci.yml)
[![Version & Release](https://github.com/davegarvey/grubble/actions/workflows/version.yml/badge.svg)](https://github.com/davegarvey/grubble/actions/workflows/version.yml)
[![Version](https://img.shields.io/github/v/release/davegarvey/grubble)](https://github.com/davegarvey/grubble/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Automatic semantic versioning from conventional commits. Designed to be driven by AI-generated commit messages.

## Why

Grubble reads your commit history, applies conventional commit rules, and bumps the version in one command:

- `feat:` → minor, `fix:` → patch, `!` / `BREAKING CHANGE` → major
- Optionally writes the new version to `Cargo.toml` or `package.json`
- Optionally generates a `CHANGELOG.md` (Keep a Changelog format)
- Cuts and pushes tags, including floating `v4` / `v4.1` tags for GitHub Actions
- Plays well with AI agents that emit conventional commits — see [`.github/prompts/sc.prompt.md`](.github/prompts/sc.prompt.md)

## Quick Start

```bash
# Install
cargo install grubble

# Make some conventional commits
git commit -m "feat: add login"
git commit -m "fix: handle empty input"

# Preview the next version
grubble --dry-run

# Release
grubble --push --tag
```

For CI on a protected branch, jump to [Releasing on a Git Server](#releasing-on-a-git-server-recommended-release-please-flow).

## Installation

### Pre-built binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/davegarvey/grubble/releases), or grab it directly:

```bash
# Linux x86_64
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-linux-x86_64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# Linux ARM64
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-linux-aarch64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# macOS Intel
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-macos-x86_64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# macOS Apple Silicon
curl -L https://github.com/davegarvey/grubble/releases/latest/download/grubble-macos-aarch64.tar.gz | tar xz
sudo mv grubble /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri https://github.com/davegarvey/grubble/releases/latest/download/grubble-windows-x86_64.zip -OutFile grubble.zip
Expand-Archive grubble.zip
```

### Cargo

```bash
cargo install grubble
```

### From source

```bash
git clone https://github.com/davegarvey/grubble.git
cd grubble
cargo build --release
# Binary at target/release/grubble
```

### GitHub Action

```yaml
- uses: davegarvey/grubble@v4
```

The Action exposes three outputs for downstream steps:

| Output | Description |
| --- | --- |
| `version` | The new version (e.g. `1.2.3`). If no bump was needed, this matches `previous-version`. |
| `previous-version` | The version before the bump. |
| `bump-type` | One of `major`, `minor`, `patch`, `none`. |

When no bump is needed (e.g. only `refactor:` or `chore:` commits since the last tag), the Action exits cleanly with `bump-type=none` rather than failing.

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

Every flag has a corresponding option in `.versionrc.json` (see [Configuration](#configuration)). CLI flags override file values.

## Configuration

Grubble reads `.versionrc.json` from the project root. Flags and file values are merged, with flags winning.

```json
{
  "preset": "rust",
  "packageFiles": ["Cargo.toml"],
  "tagPrefix": "v",
  "commitPrefix": "chore(release): bump",
  "tag": true,
  "push": true,
  "changelog": true
}
```

| Option | CLI flag | Default | Description |
| --- | --- | --- | --- |
| `preset` | `--preset` | `git` | Versioning strategy: `git`, `rust`, or `node`. |
| `packageFiles` | `--package-files` | `[]` | Comma-separated files to update (for `rust` / `node`). |
| `tagPrefix` | `--tag-prefix` | `v` | Prefix for git tags. |
| `commitPrefix` | `--commit-prefix` | `chore: bump version` | Prefix for the bump commit message. |
| `tag` | `--tag` | `false` | Create a git tag for the new version. |
| `push` | `--push` | `false` | Push the commit (and tag) to the remote. |
| `releaseNotes` | `--release-notes` (`-r`) | `false` | Include release notes in the tag annotation. Requires `tag: true`. |
| `changelog` | `--changelog` | `false` | Generate or update `CHANGELOG.md`. |
| `updateMajorTag` | `--update-major-tag` | `false` | Maintain a floating `v4` tag pointing to the latest `v4.x.x`. |
| `updateMinorTag` | `--update-minor-tag` | `false` | Maintain a floating `v4.1` tag pointing to the latest `v4.1.x`. |
| `gitUserName` | `--git-user-name` | `github-actions[bot]` | Identity used for the bump commit when no local git user is configured. |
| — | `--git-branch` | `""` | Push the bump to this branch instead of HEAD. See [Releasing on a Git Server](#releasing-on-a-git-server-recommended-release-please-flow). |
| `gitUserEmail` | `--git-user-email` | `41898282+github-actions[bot]@users.noreply.github.com` | Email used for the bump commit when no local git user is configured. |
| `types` | — | see [Commit Types](#commit-types) | Per-type bump behavior. Valid values: `major`, `minor`, `patch`, `none`. |

If your repo has a local `user.name` / `user.email` set, grubble uses those and ignores `gitUserName` / `gitUserEmail`. In CI, set these to match your bot user (e.g. `github-actions[bot]`).

## Versioning Strategies

The `preset` option controls what files grubble writes.

- **`git`** (default) — tracks versions via tags only. No files are modified. Use this for monorepos or projects with their own versioning scheme.
- **`rust`** — updates the `version` field in `Cargo.toml` and refreshes `Cargo.lock`. Pairs with `cargo publish`.
- **`node`** — updates the `version` field in `package.json` and `package-lock.json`. Pairs with `npm publish`.

When switching from `git` to a file-based preset, or when a package file is behind the latest tag, grubble first syncs the file to the tag (with a `chore: sync package version to v...` commit) and then proceeds with the normal bump.

## Major / Minor Tag Tracking

When `updateMajorTag` is enabled, grubble maintains a lightweight tag that follows the latest release in its range:

- `v4` → latest `v4.x.x`
- `v4.1` → latest `v4.1.x` (additionally requires `updateMinorTag`)

This is the convention GitHub Actions use, so consumers can reference `uses: owner/repo@v4` and automatically get the latest v4 release.

```bash
grubble --tag --push --update-major-tag --update-minor-tag
```

These tags are force-pushed. Anyone who has them checked out locally will need to re-fetch. Only enable this if you follow semver strictly — a breaking change bumps the major and consumers pinned to the old `v4` will pick it up.

## Changelog Generation

`grubble --changelog` writes a `CHANGELOG.md` in [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format:

- `feat:` → **Added**
- `fix:` → **Fixed**
- `perf:`, `refactor:` → **Changed**
- `revert:` → **Removed**
- `security:` → **Security**
- Breaking changes → **Changed** with a **BREAKING:** prefix

The changelog is committed as part of the release.

## Releasing on a Git Server (Recommended: release-please flow)

Grubble's recommended release flow is the same one used by [`semantic-release`](https://github.com/semantic-release/semantic-release) (23.9k ⭐) and [`googleapis/release-please`](https://github.com/googleapis/release-please) (7.2k ⭐, Google-maintained): open a release PR, let a human merge it, then create the tag on the merge commit. This pattern works on **any** git server (GitHub, GitLab, Gitea, etc.) — the only thing that varies is the API you call to create the PR, the tag, and the release. The grubble binary itself is server-agnostic; only the workflow glue changes.

### Why this pattern

- **Tag is on the main commit, by construction.** No orphaning risk — there is no `release/v<version>`-branch tip that a squash merge could detach the tag from.
- **Human-in-the-loop.** A release is a high-impact event; a maintainer reviews the PR before it merges.
- **Survives squash vs. merge commit.** Both styles produce a single, stable commit on main that the tag can point to.
- **Works on protected branches without a bypass token.** The only push is to a `release/v<version>` branch, which you can configure to be un-protected (the release PR is the review gate, not the branch).
- **Bot-approval-friendly.** GitHub will not let a bot-created PR approve itself for merge. With a human in the loop, this is a non-issue; with `auto-merge: true` on a bot-created PR it is a hard stop (see [Troubleshooting](#troubleshooting)).

### The flow

1. A conventional commit (e.g. `feat:`, `fix:`) lands on `main`.
2. Your release workflow runs `grubble --dry-run` to compute the next version. If it would change, the workflow opens (or updates) a release PR on a `release/v<version>` branch. The PR contains the version bump commit and the CHANGELOG entry. **No auto-merge.**
3. A maintainer reviews the release PR and merges it (squash or merge commit — both work).
4. On the next push to `main`, the workflow detects the merged release PR and creates the `v<version>` tag and a GitHub Release on the merge commit. The `v<major>` floating tag is also updated.

### `--release-from-pr`: the missing piece

The new piece in this flow is the post-merge tag step. You need a way to: (1) take a PR number, (2) confirm it was merged, (3) read the version from the head branch, and (4) get the merge commit SHA. That is exactly what `grubble --release-from-pr <NUMBER>` does:

```bash
$ grubble --release-from-pr 79 --output json
{
  "body": "Automated release PR created by grubble.",
  "major_tag_name": "v5",
  "merge_commit_sha": "582643110c4fa6e5279f9f6705fbbc1bf1e52143",
  "tag_name": "v5.2.2",
  "title": "Release v5.2.2",
  "version": "5.2.2"
}
```

`--release-from-pr` is a **read-only** operation: it queries the GitHub API for the PR, validates the head branch matches `^release/v\d+\.\d+\.\d+$`, validates the PR is merged, and emits the spec. It does NOT create tags, releases, or anything else. The split keeps grubble's job small (resolve the PR) and your workflow's job explicit (decide what to do with the resolved spec — cut the tag, write a release, post to Slack, etc.).

It is mutually exclusive with the bump modes (`--push`, `--tag`, `--changelog`, etc.) because it is a read-only resolution, not a state-changing bump.

### GitHub Actions: release-please flow with the grubble Action

The grubble Action supports this flow via two inputs that work together: `create-pr: true` (with no `auto-merge`) opens the release PR, and `release-from-pr: <NUMBER>` resolves a merged release PR to a tag spec. The two are used in different invocations of the Action — the bump and the post-merge resolution are different shapes, so they don't compose in a single `uses:` step.

```yaml
name: Version & Release
on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  version:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      # 1. Open or update the release PR. Bump + push the release branch
      #    (no --tag) and open the PR. No auto-merge — a human reviews.
      - uses: davegarvey/grubble@v5
        with:
          push: true
          create-pr: true

      # 2. Detect a merged release PR.
      - name: Detect merged release PR
        id: detect
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          PR_INFO=$(gh pr list --state merged --base main --limit 20 \
            --json number,headRefName,mergeCommit,mergedAt \
            | jq -r '[.[] | select(.headRefName | test("^release/v[0-9]+\\.[0-9]+\\.[0-9]+$"))] | sort_by(.mergedAt) | last // empty')
          if [ -n "$PR_INFO" ] && [ "$PR_INFO" != "null" ]; then
            PR_NUMBER=$(echo "$PR_INFO" | jq -r '.number')
            MERGE_SHA=$(echo "$PR_INFO" | jq -r '.mergeCommit.oid')
            echo "merged=true" >> $GITHUB_OUTPUT
            echo "pr_number=$PR_NUMBER" >> $GITHUB_OUTPUT
            echo "merge_sha=$MERGE_SHA" >> $GITHUB_OUTPUT
          else
            echo "merged=false" >> $GITHUB_OUTPUT
          fi

      # 3. Resolve the merged PR to a tag spec via the grubble Action.
      #    The Action's `release-from-pr` input runs `grubble --release-from-pr`
      #    in read-only mode and emits the spec as `release-spec`.
      - name: Resolve release spec
        id: resolve
        if: steps.detect.outputs.merged == 'true'
        uses: davegarvey/grubble@v5
        with:
          release-from-pr: ${{ steps.detect.outputs.pr_number }}

      # 4. Cut the tag + GitHub Release on the merge commit.
      - name: Release merged PR
        if: steps.detect.outputs.merged == 'true'
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          # Use fromJSON() to safely parse the multi-line JSON spec from
          # the Action's output. Shell-quoting JSON directly is fragile when
          # the PR body contains apostrophes or other shell-special chars.
          TAG='${{ fromJSON(steps.resolve.outputs.release-spec).tag_name }}'
          MAJOR='${{ fromJSON(steps.resolve.outputs.release-spec).major_tag_name }}'
          SHA='${{ fromJSON(steps.resolve.outputs.release-spec).merge_commit_sha }}'
          BODY='${{ fromJSON(steps.resolve.outputs.release-spec).body }}'

          if [ -z "$TAG" ] || [ "$TAG" = "null" ]; then
            echo "::error::Failed to resolve release for PR"
            exit 1
          fi

          # Tag (idempotent: skip if it already exists on this commit).
          if ! gh api "repos/${GITHUB_REPOSITORY}/git/refs/tags/${TAG}" >/dev/null 2>&1; then
            gh api -X POST -H "Accept: application/vnd.github+json" \
              "repos/${GITHUB_REPOSITORY}/git/refs" \
              -f "ref=refs/tags/${TAG}" -f "sha=${SHA}"
          fi

          # Floating major tag (always force).
          gh api -X PATCH -H "Accept: application/vnd.github+json" \
            "repos/${GITHUB_REPOSITORY}/git/refs/tags/${MAJOR}" \
            -f "sha=${SHA}" -F "force=true" >/dev/null

          # GitHub Release (idempotent: skip if it exists).
          if ! gh release view "${TAG}" >/dev/null 2>&1; then
            gh release create "${TAG}" --target "${SHA}" \
              --title "${TAG}" --notes "${BODY}" --verify-latest
          fi
```

That is the whole thing. The Action's `create-pr` input (no `auto-merge`) opens the release PR; a human merges it; the next push to `main` runs the Action's `release-from-pr` mode to resolve the spec, and the `Release merged PR` step cuts the tag and creates the GitHub Release on the merge commit.

### Reference implementation: grubble's own version.yml

The repo's own release workflow (`.github/workflows/version.yml`) is the canonical example. It runs the Bump step (`grubble --raw --dry-run`) to compute the next version, opens or updates the release PR (no auto-merge), and on the next push after merge runs `--release-from-pr` to resolve the merged PR and create the tag + GitHub Release on the merge commit via `gh api`. v5.2.0, v5.2.1, and v5.2.2 were all released through this flow.

## Best Practices

Patterns and pitfalls, distilled from the release-please / semantic-release playbook and from grubble's own CI. Read this once before designing your release workflow.

### Releases

- **Use the release-please flow** for any project on a protected branch. The release-please flow is the recommended pattern because it gives you a human review gate, an audit trail, survives squash vs. merge commit, and works on protected branches without a bypass token.
- **Do not enable `auto-merge` on the release PR** if your branch protection requires review approval. GitHub will not let a bot-created PR approve itself, so the PR will sit in "Waiting for approval" indefinitely. Let a human click "Merge" on the release PR — that is the security model.
- **The Open and Release steps in the version workflow are independent.** The Open step runs whenever a new release is needed (Bump step reports `changed=true`); the Release step runs whenever a previously-merged release PR is detected. They do not race, and decoupling them is what makes the workflow correct after a "quiet period" (when no release PRs have been merged for a while). If you copy the workflow into your own repo, do not re-introduce a gating dependency between them.
- **Tags are created on the main merge commit, not on the release branch tip.** This is the canonical pattern and it eliminates tag-orphaning on squash merges. Pre-tagging the release branch (the older grubble default) is a known footgun — the tag is at risk of being detached from `main` after a squash merge.
- **Floating major tags (`v5`) and minor tags (`v5.2`) are force-moved on every release.** Consumers who pin to `owner/repo@v5` automatically get the latest v5.x.x. Only enable `--update-major-tag` if you follow semver strictly — a breaking change bumps the major and silently moves the tag for everyone.
- **Use the `release-from-pr` Action input in your post-merge step.** It is the read-only resolution step that turns a merged release PR into a tag spec (`version`, `tag_name`, `major_tag_name`, `merge_commit_sha`, `title`, `body`) without writing any state. Your workflow then does the `gh api` calls to create the tag and GitHub Release. This is cleaner than shelling out to `grubble --release-from-pr` in a `run:` step, because the Action handles binary download in each invocation.

- **Force-push the release branch with `--force-with-lease`, not plain `--force`.** The release branch is owned by the workflow — it is re-created from `main` on every push, so it must be force-pushable. Use `--force-with-lease` (via `--force-push`) instead of plain `--force` because it fails if the remote has been updated since the local checkout (e.g., a human pushed a fix to the branch). This matches `release-please`'s own use of `--force-with-lease`. The `--force-push` flag requires `--git-branch` and is rejected if you try to use it on the current branch.

### CI gating

- **Use `--raw --dry-run` (or `--bump-type`) to gate bumps in CI, not exit codes.** `--dry-run` always exits 0 in v5+, so it can no longer be used as a "would a bump happen?" signal. Use `--raw --dry-run` (returns just the next version) and compare to the current `Cargo.toml` / `package.json` version, or use `--bump-type` (returns `major` / `minor` / `patch` / `none`) for a clean gate.
- **Use `--output json` from CI scripts that need to parse the result.** Both `--bump-type` and `--raw` accept `--output json` for stable, machine-readable output. Avoid shell-substring matching on the human-readable output.
- **Pin the major version (`@v5`) unless you want the release frozen.** The floating major tag follows the latest release in its range. If you want a frozen release, pin to a specific version (`@v5.2.2`).

### Commits

- **Use conventional commit messages** (`feat:`, `fix:`, `perf:`, `refactor:`, `docs:`, `chore:`, etc.). Grubble reads the commit history to compute the next version, and the commit type determines the bump (see [Commit Types](#commit-types) for the default mapping).
- **Mark breaking changes with `!` or a `BREAKING CHANGE:` footer.** A `!` after the type/scope (e.g. `feat!: rewrite auth`) or a `BREAKING CHANGE: <description>` line in the commit body triggers a major bump. Anything else in `feat:` or `fix:` only triggers a minor or patch bump respectively.
- **Do not edit a release PR after it has been opened.** Push additional fix: commits to `main` and let the workflow update the release PR by force-pushing the release branch. Editing the PR file directly via the GitHub UI creates a commit on the release branch that is not in the version.yml's commit analysis, leading to a mismatch.
- **Squash-merge your PRs.** Grubble's commit analysis is per-commit, not per-PR. A `feat:` squashed into a `fix:` commit becomes a `fix:` commit and only triggers a patch bump. Use squash-merge (or rebase-merge) to keep individual commits in their conventional format.

### Pinned versions

- **For your own release workflow, use a specific tag rather than a branch.** Pin `davegarvey/grubble@v5.2.2` (not `@v5`) for stability. The major tag (`@v5`) is for end-user consumption where "latest patch" is the desired behavior.
- **If you are publishing a binary that other projects depend on, use `--update-major-tag` and the major tag will be your public contract.** Treat moving the major tag as a breaking change: communicate it, version it, and give consumers time to migrate.

## Direct-Push Style (Alternative)

If your `main` is **not** protected, or you have configured your repo to allow `GITHUB_TOKEN` to bypass branch protection, the simplest possible flow is direct-push: a workflow that triggers on every push to `main`, runs `grubble --push --tag`, and ships. This is the same pattern grubble shipped in v4 and earlier — it still works, but we recommend the release-please flow above for any new project because it gives you a human review gate, an audit trail, and works on protected branches.

> Skip this section if your `main` is protected with required reviews and you want them enforced. The release-please flow above is the right tool for that.

### GitHub Action: direct-push

```yaml
name: Release
on:
  pull_request:
    types: [closed]
    branches: [main]

jobs:
  release:
    if: github.event.pull_request.merged == true
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: read
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: davegarvey/grubble@v5
        with:
          push: true
          tag: true
          update-major-tag: true
```

The `davegarvey/grubble@v5` pin floats to the latest v5.x.x release. Pin to a specific tag (e.g. `@v5.2.2`) if you need the release frozen.

### Manual setup (direct-push)

Use this when you want to run tests and linting before the bump, or when self-hosted runners can't reach GitHub Releases.

```yaml
name: Release
on:
  pull_request:
    types: [closed]
    branches: [main]

jobs:
  release:
    if: github.event.pull_request.merged == true
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: read
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - name: Bump version and release
        run: |
          grubble \
            --preset rust \
            --push \
            --tag \
            --update-major-tag \
            --git-user-name "github-actions[bot]" \
            --git-user-email "41898282+github-actions[bot]@users.noreply.github.com"
```

CI checklist:

- Grant `contents: write` so the workflow can push commits and tags.
- Use `fetch-depth: 0` on `actions/checkout` so grubble can see the full history.
- Pin the major version (`@v5`) unless you want the release frozen.
- Use the GitHub Action unless you need custom logic between the lint and the bump.

## Bypass Token (Advanced)

If your `main` branch is protected (required PRs, required status checks), the default `push: true` will fail with `GH006: Protected branch update failed` because `GITHUB_TOKEN` cannot bypass branch protection.

The recommended pattern is the canonical **release-please** flow: open a release PR, let a human merge it, then create the tag on the merge commit. This is the same pattern used by [`semantic-release`](https://github.com/semantic-release/semantic-release) (23.9k ⭐) and [`release-please`](https://github.com/googleapis/release-please) (7.2k ⭐, Google-maintained). The key properties:

- **Tag is on the main commit, by construction.** No orphaning risk.
- **Human-in-the-loop.** A release is a high-impact event; a maintainer reviews the PR before it merges.
- **No auto-merge quirks.** GitHub's auto-merge bot pushes don't always trigger downstream workflows, and bot-created PRs need CI approval. With the human in the loop, this is a non-issue.

### The flow

1. A conventional commit (e.g. `feat:`, `fix:`) lands on `main`.
2. Your release workflow runs `grubble --dry-run` to compute the next version. If it would change, the workflow opens (or updates) a release PR on a `release/v<version>` branch. The PR contains the version bump commit and the CHANGELOG entry. **No auto-merge.**
3. A maintainer reviews the release PR and merges it (squash or merge commit — both work).
4. On the next push to `main`, the workflow detects the merged release PR and creates the `v<version>` tag and a GitHub Release on the merge commit via the GitHub API. The `v<major>` floating tag is also updated.

### Reference implementation

Grubble's own release workflow (`.github/workflows/version.yml`) implements this pattern. The shape:

```yaml
- name: Bump (dry-run)
  id: bump
  run: |
    VERSION_BEFORE=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
    VERSION_AFTER=$(./grubble --preset rust --dry-run)
    # ...compare and emit outputs...

- name: Open or update release PR
  if: steps.bump.outputs.changed == 'true' && steps.detect.outputs.merged != 'true'
  run: |
    # Bump Cargo.toml + CHANGELOG.md, push the release branch, open the PR.
    # NO --tag here — the tag is created on the main merge commit by the
    # "Release merged PR" step.

- name: Release merged PR
  id: release
  if: steps.detect.outputs.merged == 'true'
  run: |
    # Resolve the merged PR to a tag spec via `grubble --release-from-pr`,
    # then create the v<version> tag and GitHub Release on the merge
    # commit via the GitHub API.
```

The grubble `--release-from-pr` flag is the small piece of binary logic the workflow calls to resolve a PR to a tag spec:

```bash
grubble --release-from-pr 79 --output json
# {
#   "body": "Automated release PR created by grubble.",
#   "major_tag_name": "v5",
#   "merge_commit_sha": "5826431...",
#   "tag_name": "v5.2.2",
#   "title": "Release v5.2.2",
#   "version": "5.2.2"
# }
```

### Direct-push style (for unprotected branches)

If your `main` is not protected, the simplest approach is `grubble --push --tag` from CI on every push to `main`. The CLI flags unchanged from earlier versions:

```yaml
- name: Bump and push
  run: |
    grubble --push --tag --update-major-tag \
      --preset rust \
      --git-user-name "github-actions[bot]" \
      --git-user-email "41898282+github-actions[bot]@users.noreply.github.com"
```

This is the simplest possible flow. Use it when your default branch is not protected, or when you've already set up branch protection to allow the GITHUB_TOKEN to bypass (see below).

### Bypass token (advanced)

If you cannot use the PR flow and must push directly to a protected branch (for example, to make an existing direct-push workflow work after enabling branch protection), supply a custom token via the Action's `token` input:

```yaml
- uses: davegarvey/grubble@v5
  with:
    push: true
    token: ${{ secrets.RELEASE_PAT }}
```

The Action masks the token in logs and rewrites the remote URL with it before grubble runs. The token must have push access and bypass privileges for the protected branch — typically a GitHub App installation token or a fine-grained PAT with `contents: write`.

**Prefer the release-please flow whenever possible.** Bypassing branch protection skips required reviews and status checks, which undermines the protections you set up. The bypass token should be a last resort, not a default.

## CI/CD Patterns

### Skip the bump when nothing changed

`grubble --bump-type` prints `major`, `minor`, `patch`, or `none`. Use it as a gate:

```bash
if [ "$(grubble --bump-type --preset rust)" != "none" ]; then
  grubble --push --tag --preset rust
fi
```

`grubble --dry-run` always exits 0 (success, including no-op). For the "would a bump happen?" signal, use `--bump-type`.

### Read the bump type

`grubble --bump-type` prints `major`, `minor`, `patch`, or `none`. Use it to drive conditional logic:

```bash
case "$(grubble --bump-type)" in
  major) notify "breaking release" ;;
  minor) notify "feature release" ;;
  patch) : ;;                  # silent patch
  none)   exit 0 ;;
esac
```

### Read the new version

`grubble --raw` prints the version that would be released without making changes. It honors `--preset` (e.g. `--preset rust` reads from `Cargo.toml`; `--preset node` reads from `package.json`; `--preset git` reads from the latest tag). Pair it with `gh release create` or a deploy step:

```bash
NEW_VERSION=$(grubble --raw --preset rust)
gh release create "v$NEW_VERSION" --title "Release v$NEW_VERSION"
```

### Machine-readable output

Both `--bump-type` and `--raw` accept `--output json` for stable, machine-readable output:

```bash
$ grubble --bump-type --output json
{
  "bump_type": "minor",
  "current_version": "1.2.3",
  "triggering_commits": ["Minor: feat: add login"],
  "unknown_commits": []
}

$ grubble --raw --preset rust --output json
{
  "version": "1.2.3",
  "preset": "rust"
}
```

`--output json` is rejected when combined with the normal run mode or `--dry-run` (those modes always print human-readable text). Use `--output json` from CI scripts that need to parse the result instead of shell-substring matching.

## How It Works

1. Sync package files to the latest tag (for `rust` / `node` presets).
2. Analyze commits since the last tag using conventional commit rules.
3. Pick the highest bump (`major` / `minor` / `patch` / `none`).
4. Update package files and `CHANGELOG.md` if configured.
5. Create the bump commit.
6. Create tags, including `v4` / `v4.1` floating tags if enabled.
7. Push to the remote if configured.

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

## Troubleshooting

**"Author identity unknown"**

Set a git identity in the workflow before running grubble:

```yaml
- run: |
    git config user.name "github-actions[bot]"
    git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
```

The `--git-user-name` and `--git-user-email` flags are ignored when a local `user.name` / `user.email` is already set.

**"grubble: command not found"**

Make sure grubble is on `PATH`. If you used `cargo install --root <dir>`, add `<dir>/bin` to `PATH`.

**No version bump on merge**

- Confirm the merged commits include `feat:` / `fix:` (or another type mapped to a bump).
- Confirm the workflow has `contents: write`.
- Confirm `actions/checkout` uses `fetch-depth: 0`.

**Invalid config file**

Empty or invalid `.versionrc.json` falls back to defaults with a warning. Run grubble once to see the warning, then fix the JSON.

**Package file not found**

- Check that `--package-files` points to files relative to the repo root.
- For multi-package repos, pass each file: `--package-files "Cargo.toml,client/Cargo.toml"`.

**Push fails on protected branches (`GH006`)**

`GITHUB_TOKEN` cannot bypass branch protection. Use the [release-please flow](#releasing-on-a-git-server-recommended-release-please-flow) (`branch` + `create-pr`, no `auto-merge`), or supply a [bypass token](#bypass-token-advanced) if you must push directly.

**Auto-merge PR is stuck in "Waiting for approval"**

GitHub does not allow a bot-created PR to approve itself. If your branch protection rule requires review approval, a PR opened by `create-pr: true` (which uses `GITHUB_TOKEN`) cannot satisfy the approval check and `auto-merge: true` will never trigger. The fix is to remove `auto-merge` and have a human click "Merge" on the release PR — i.e. use the [release-please flow](#releasing-on-a-git-server-recommended-release-please-flow). This is the same reason grubble's own `version.yml` workflow intentionally does **not** call `gh pr merge --auto`.

**"Invalid format" in `$GITHUB_OUTPUT`**

Fixed in v4.9.4 (see [#54](https://github.com/davegarvey/grubble/issues/54)). Bump the Action to `davegarvey/grubble@v4` (movable) to pick up the fix, or pin to a release ≥ v4.9.4 once you have a chance to verify.

## Migration from v4

### v5.0.0 — CLI contract change

v5.0.0 is a breaking release for the CLI's exit-code contract. The GitHub Action and `version.yml` workflow are unchanged in behavior at the v5.0 boundary; the v5 contract is what they always wanted.

**What changed in v5.0.0:**

- **`grubble` exits 0 on success**, including when no bump is needed. Previously, exit 1 meant "no bump" — now exit 1 means "error."
- **`grubble --raw` honors `--preset`**. `--raw --preset rust` reads from `Cargo.toml`; `--raw --preset node` reads from `package.json`; `--raw --preset git` reads from the latest tag. Previously, `--raw` always read from git tags regardless of preset.
- **New `--output text|json` flag** for `--bump-type` and `--raw`. Use this from CI scripts that need to parse the result.
- **Action requires release checksums** (was warn-and-continue on missing `.sha256`).
- The Action's "Get current version" step now uses a single `./grubble --raw --preset <preset>` call instead of preset-specific shell branches.

### v5.2.0 — release workflow restructured (recommended reading)

v5.2.0 switched the canonical `version.yml` workflow to the release-please flow. If you copied grubble's `version.yml` into your own repo and now need to understand what changed, or if you want to migrate from a pre-merge-tag setup, read [Releasing on a Git Server](#releasing-on-a-git-server-recommended-release-please-flow) — that section is the recommended pattern for any repo on a protected branch.

**The key behavioral change:** tags are no longer created on the release branch commit. They are created on the **main merge commit** after a human merges the release PR. This eliminates tag-orphaning on squash merges and means the tag is on a single, stable commit by construction.

**New CLI flag:** `grubble --release-from-pr <NUMBER>` resolves a merged release PR to a tag spec (`version`, `tag_name`, `major_tag_name`, `merge_commit_sha`, `title`, `body`). It is the read-only resolution step your post-merge workflow calls before creating the tag and GitHub Release via `gh api`. It is mutually exclusive with the bump modes (it does not modify any state).

**New Action input:** `release-from-pr: <NUMBER>` on the grubble Action runs the same flag and emits the spec as the `release-spec` output. Use it in a post-merge step when you want the Action to do the resolution for you (see the reference workflow in [Releasing on a Git Server](#releasing-on-a-git-server-recommended-release-please-flow)).

### If you script against the CLI

Replace exit-code gates with `--bump-type` checks:

```bash
# v4 (broken on v5)
if grubble --dry-run --preset rust; then
  grubble --push --tag --preset rust
fi

# v5
if [ "$(grubble --bump-type --preset rust)" != "none" ]; then
  grubble --push --tag --preset rust
fi
```

Replace `--raw` exit-code handling with output parsing:

```bash
# v4
NEW_VERSION=$(./grubble --raw --preset rust) || NEW_VERSION=$(./grubble --raw --preset rust || echo "0.0.0")

# v5 — exits 0 whenever a version is produced
NEW_VERSION=$(./grubble --raw --preset rust)
```

### Staying on v4

`@v4` (the floating major tag) and all `@v4.x.x` specific tags remain available indefinitely. Pin to `@v4` or `@v4.9.4` to stay on the v4 contract. The default floating tag shifts to `@v5` once v5 ships.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, commit guidelines, and the release process.

## License

[MIT](LICENSE).

## For AI Users

This tool is built for AI-generated commits that follow the conventional commit format. See [`.github/prompts/sc.prompt.md`](.github/prompts/sc.prompt.md) for a prompt that produces commits compatible with grubble.
