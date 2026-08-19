# Troubleshooting

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

Empty or invalid `.versionrc.json` falls back to defaults with a warning during normal runs. The warning includes the JSON parse error. In CI, run `grubble --validate-config` to fail fast before a release if the file is invalid.

**Package file not found**

- Check that `--package-files` points to files relative to the repo root.
- For multi-package repos, pass each file: `--package-files "Cargo.toml,client/Cargo.toml"`.

**Push fails on protected branches (`GH006`)**

`GITHUB_TOKEN` cannot bypass branch protection. Use the [release-please flow](release-workflow.md) (`branch` + `create-pr`, no `auto-merge`), or supply a [bypass token](release-workflow.md#bypass-token-advanced) if you must push directly.

**Auto-merge PR is stuck in "Waiting for approval"**

GitHub does not allow a bot-created PR to approve itself. If your branch protection rule requires review approval, auto-merge on the release PR will never trigger. Disable auto-merge and have a human click "Merge" on the release PR. Grubble's own workflow enables auto-merge because its branch protection does not require review approval (only the `test` check is required).

**"head branch is not up to date with the base branch"**

Branch protection requires the PR branch to be up to date with `main` before merging. If `gh pr merge` fails with this message, set auto-merge and update the branch:

```bash
gh pr merge <NUMBER> --auto --squash
gh pr update-branch <NUMBER>
```

Auto-merge waits for CI to pass on the updated branch, then merges. This is the standard workflow for repos with "Require branches to be up to date" enabled.

**"Invalid format" in `$GITHUB_OUTPUT`**

Fixed in v4.9.4 (see [#54](https://github.com/davegarvey/grubble/issues/54)). Bump the Action to `davegarvey/grubble@v4` (movable) to pick up the fix, or pin to a release ≥ v4.9.4 once you have a chance to verify.
