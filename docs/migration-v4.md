# Migration from v4

## v5.0.0 — CLI contract change

v5.0.0 is a breaking release for the CLI's exit-code contract. The GitHub Action and `version.yml` workflow are unchanged in behavior at the v5.0 boundary; the v5 contract is what they always wanted.

**What changed in v5.0.0:**

- **`grubble` exits 0 on success**, including when no bump is needed. Previously, exit 1 meant "no bump" — now exit 1 means "error."
- **`grubble --raw` honors `--preset`**. `--raw --preset rust` reads from `Cargo.toml`; `--raw --preset node` reads from `package.json`; `--raw --preset git` reads from the latest tag. Previously, `--raw` always read from git tags regardless of preset.
- **New `--output text|json` flag** for `--bump-type` and `--raw`. Use this from CI scripts that need to parse the result.
- **Action requires release checksums** (was warn-and-continue on missing `.sha256`).
- The Action's "Get current version" step now uses a single `./grubble --raw --preset <preset>` call instead of preset-specific shell branches.

## v5.2.0 — release workflow restructured (recommended reading)

v5.2.0 switched the canonical `version.yml` workflow to the release-please flow. If you copied grubble's `version.yml` into your own repo and now need to understand what changed, or if you want to migrate from a pre-merge-tag setup, read [Release Workflow](release-workflow.md) — that page is the recommended pattern for any repo on a protected branch.

**The key behavioral change:** tags are no longer created on the release branch commit. They are created on the **main merge commit** after a human merges the release PR. This eliminates tag-orphaning on squash merges and means the tag is on a single, stable commit by construction.

**New CLI flag:** `grubble --release-from-pr <NUMBER>` resolves a merged release PR to a tag spec (`version`, `tag_name`, `major_tag_name`, `merge_commit_sha`, `title`, `body`). It is the read-only resolution step your post-merge workflow calls before creating the tag and GitHub Release via `gh api`. It is mutually exclusive with the bump modes (it does not modify any state).

**New Action input:** `release-from-pr: <NUMBER>` on the grubble Action runs the same flag and emits the spec as the `release-spec` output. Use it in a post-merge step when you want the Action to do the resolution for you (see the reference workflow in [Release Workflow](release-workflow.md)).

## If you script against the CLI

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

## Staying on v4

`@v4` (the floating major tag) and all `@v4.x.x` specific tags remain available indefinitely. Pin to `@v4` or `@v4.9.4` to stay on the v4 contract. The default floating tag shifts to `@v5` once v5 ships.
