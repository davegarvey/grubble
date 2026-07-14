# Release Workflows

Grubble supports two release workflows. The **release-please flow** is recommended for any project on a protected branch.

## Release-Please Flow (Recommended)

This is the same pattern used by [`semantic-release`](https://github.com/semantic-release/semantic-release) (23.9k ⭐) and [`googleapis/release-please`](https://github.com/googleapis/release-please) (7.2k ⭐, Google-maintained): open a release PR, let a human merge it, then create the tag on the merge commit.

### Why this pattern

- **Tag is on the main commit, by construction.** No orphaning risk — there is no `release/v<version>`-branch tip that a squash merge could detach the tag from.
- **Human-in-the-loop.** A release is a high-impact event; a maintainer reviews the PR before it merges.
- **Survives squash vs. merge commit.** Both styles produce a single, stable commit on main that the tag can point to.
- **Works on protected branches without a bypass token.** The only push is to a `release/v<version>` branch, which you can configure to be un-protected (the release PR is the review gate, not the branch).
- **Bot-approval-friendly.** GitHub will not let a bot-created PR approve itself for merge. With a human in the loop, this is a non-issue; with `auto-merge: true` on a bot-created PR it is a hard stop (see [Troubleshooting](troubleshooting.md)).

### The flow

1. A conventional commit (e.g. `feat:`, `fix:`) lands on `main`.
2. Your release workflow runs `grubble --dry-run` to compute the next version. If it would change, the workflow opens (or updates) a release PR on a `release/v<version>` branch. The PR contains the version bump commit and the CHANGELOG entry. **No auto-merge.**
3. A maintainer reviews the release PR and merges it (squash or merge commit — both work).
4. On the next push to `main`, the workflow detects the merged release PR and creates the `v<version>` tag and a GitHub Release on the merge commit. The `v<major>` floating tag is also updated.

### `--release-from-pr`: the missing piece

The new piece in this flow is the post-merge tag step. You need a way to: (1) take a PR number, (2) confirm it was merged, (3) read the version from the head branch, and (4) get the merge commit SHA. That is exactly what `grubble --release-from-pr <NUMBER>` does:

```bash
grubble --release-from-pr 79 --output json
# {
#   "body": "## [5.2.2] - 2026-07-14\n\n### Fixed\n\n- use merge commit so release tag stays reachable",
#   "major_tag_name": "v5",
#   "merge_commit_sha": "582643110c4fa6e5279f9f6705fbbc1bf1e52143",
#   "tag_name": "v5.2.2",
#   "title": "Release v5.2.2",
#   "version": "5.2.2"
# }
```

`--release-from-pr` is a **read-only** operation: it queries the GitHub API for the PR, validates the head branch matches `^release/v\d+\.\d+\.\d+$`, validates the PR is merged, and emits the spec. It does NOT create tags, releases, or anything else. The split keeps grubble's job small (resolve the PR) and your workflow's job explicit (decide what to do with the resolved spec — cut the tag, write a release, post to Slack, etc.).

It is mutually exclusive with the bump modes (`--push`, `--tag`, `--changelog`, etc.) because it is a read-only resolution, not a state-changing bump.

### GitHub Actions: release-please flow with the grubble Action

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
          TAG='${{ fromJSON(steps.resolve.outputs.release-spec).tag_name }}'
          MAJOR='${{ fromJSON(steps.resolve.outputs.release-spec).major_tag_name }}'
          SHA='${{ fromJSON(steps.resolve.outputs.release-spec).merge_commit_sha }}'
          BODY='${{ fromJSON(steps.resolve.outputs.release-spec).body }}'

          if ! gh api "repos/${GITHUB_REPOSITORY}/git/refs/tags/${TAG}" >/dev/null 2>&1; then
            gh api -X POST -H "Accept: application/vnd.github+json" \
              "repos/${GITHUB_REPOSITORY}/git/refs" \
              -f "ref=refs/tags/${TAG}" -f "sha=${SHA}"
          fi

          gh api -X PATCH -H "Accept: application/vnd.github+json" \
            "repos/${GITHUB_REPOSITORY}/git/refs/tags/${MAJOR}" \
            -f "sha=${SHA}" -F "force=true" >/dev/null

          if ! gh release view "${TAG}" >/dev/null 2>&1; then
            gh release create "${TAG}" --target "${SHA}" \
              --title "${TAG}" --notes "${BODY}" --verify-latest
          fi
```

### Reference implementation: grubble's own version.yml

The repo's own release workflow (`.github/workflows/version.yml`) is the canonical example. It runs the Bump step (`grubble --raw --dry-run`) to compute the next version, opens or updates the release PR (no auto-merge), and on the next push after merge runs `--release-from-pr` to resolve the merged PR and create the tag + GitHub Release on the merge commit via `gh api`.

## Direct-Push Style (Alternative)

If your `main` is **not** protected, or you have configured your repo to allow `GITHUB_TOKEN` to bypass branch protection, the simplest possible flow is direct-push: a workflow that triggers on every push to `main`, runs `grubble --push --tag`, and ships.

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

The recommended pattern is the canonical **release-please** flow described above. If you cannot use the PR flow and must push directly to a protected branch, supply a custom token via the Action's `token` input:

```yaml
- uses: davegarvey/grubble@v5
  with:
    push: true
    token: ${{ secrets.RELEASE_PAT }}
```

The Action masks the token in logs and rewrites the remote URL with it before grubble runs. The token must have push access and bypass privileges for the protected branch — typically a GitHub App installation token or a fine-grained PAT with `contents: write`.

**Prefer the release-please flow whenever possible.** Bypassing branch protection skips required reviews and status checks, which undermines the protections you set up. The bypass token should be a last resort, not a default.
