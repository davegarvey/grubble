## Approach

Add a new CLI flag `--force-push` that, when set in combination with `--git-branch <NAME>`, switches the underlying `git push` to `git push --force-with-lease origin <NAME>`. When set without `--git-branch`, the binary errors out at the arg-validation layer with a clear message. The flag is documented in `--help` and the README's Usage section, and used in the `version.yml` workflow's `Open or update release PR` step.

## Why this approach (vs. alternatives)

**Alternative A: Make `--push` always force when `--git-branch` is set.**

- Pro: simpler API (no new flag).
- Con: implicit behavior change for any user who passes both `--git-branch` and `--push` (a footgun: the user might be force-pushing without realizing it).
- Con: hard to communicate to users; the docs would have to explain "the combination of these two flags does something different."
- Reject.

**Alternative B: Make `--push` always force unconditionally (not gated to `--git-branch`).**

- Pro: simplest API.
- Con: force-pushing to `main` from a bot run is exactly what the canonical release-please flow is designed to *avoid* — the whole point of the release-please pattern is that you don't force-push `main` from CI. `--push` should remain non-force so users on unprotected branches keep their current safety guarantees.
- Reject.

**Alternative C: Add `--force-push` as a separate flag, opt-in. (Chosen.)**

- Pro: explicit. The user must opt in to force semantics.
- Pro: gated to `--git-branch` so the flag is only meaningful in the release-branch context. Matches `release-please`'s design: their release-branch push is a separate code path from the main push.
- Pro: `--force-with-lease` (not plain `--force`) is the safety variant. If a human or another workflow has updated the branch since the workflow checked it out, the push fails rather than silently overwriting. This matches `release-please`'s own use of `--force-with-lease`.
- Pro: minimal binary change — one new flag, one push variant, ~10 lines of code.
- Con: users who want force semantics have to remember the flag. Mitigated by the README's Usage section, the `--help` output, and the version.yml workflow that uses it.

## Implementation sketch

### `src/main.rs`

1. Add a new field to `Args`:

   ```rust
   /// Force-push the bump commit to the named release branch using
   /// `git push --force-with-lease`. Requires `--git-branch` to be set.
   /// This is the recommended way to keep a workflow-owned release
   /// branch in sync with `main` across multiple workflow runs
   /// (matches the `release-please` pattern).
   #[arg(long, requires = "git_branch")]
   force_push: bool,
   ```

2. In the bump flow where `--push` is processed, branch on `args.force_push`:

   ```rust
   if args.push {
       if args.force_push {
           // Use --force-with-lease for safety: if the branch was
           // updated by a concurrent push (human or another workflow),
           // this push fails rather than silently overwriting.
           git::push_branch(&branch, /* force_with_lease = */ true)?;
       } else {
           git::push_branch(&branch, /* force_with_lease = */ false)?;
       }
   }
   ```

3. Update `--help` output automatically via the doc comment.

### `src/git.rs`

Add a small wrapper around `git push` that takes a `force_with_lease: bool` parameter:

```rust
pub fn push_branch(branch: &str, force_with_lease: bool) -> BumperResult<()> {
    let mut cmd = Command::new("git");
    cmd.arg("push");
    if force_with_lease {
        cmd.arg("--force-with-lease");
    }
    cmd.arg("origin").arg(branch);
    // ... existing push logic (set GIT_ASKPASS, etc.)
}
```

The existing push code path (used by `--push` without `--git-branch`) can also route through this wrapper with `force_with_lease: false` to consolidate.

### `.github/workflows/version.yml`

Update the `Open or update release PR` step:

```yaml
git checkout -B "${BRANCH}"
./target/release/grubble \
  --git-user-name "github-actions[bot]" \
  --git-user-email "41898282+github-actions[bot]@users.noreply.github.com" \
  --git-branch "${BRANCH}" \
  --preset rust \
  --changelog \
  --push \
  --force-push
```

And replace the misleading comment with one that explains what actually happens:

```bash
# Run grumble on the release branch: bumps Cargo.toml, updates
# CHANGELOG.md, and pushes with --force-with-lease so the branch
# always tracks the current main HEAD (the previous run's branch
# is rebased-on-the-fly by recreating it from main + bump). The
# --force-push flag is required because the workflow is the sole
# writer of the release branch; the lease variant protects against
# the case where a human has pushed to the branch in the meantime.
```

### `README.md`

Add a one-line entry in the Usage block:

```bash
grubble --push --git-branch release/v0.35.0 --force-push  # same, with --force-with-lease (for release branches owned by a workflow)
```

Add a "Releasing" sub-section under "Best Practices" that explains the release-branch force-push pattern, referencing `release-please` and `semantic-release` for context.

## Test plan

1. **Unit test**: `grumble --force-push` (no `--git-branch`) errors out with a clear message.
2. **Unit test**: `grumble --force-push --git-branch <NAME>` (with no `--push`) does not error at the arg layer; the bump proceeds and the push is skipped (no `--push` means no push). The flag is only meaningful with `--push`.
3. **End-to-end** (verified by the `canonical-release-workflow` spec): create a release branch, push it via the workflow, push a non-bump commit to main, verify the workflow re-runs and the release branch is force-updated (not "behind"). This is the regression scenario from issue #89.

## Risk

- **Low.** The change adds a new flag, gated to `--git-branch`, with a clear safety variant (`--force-with-lease`). Existing users are unaffected. The version.yml workflow change is the only behavioral change, and it aligns the workflow with its stated design (and with the canonical release-please pattern).
- **The version.yml change MUST be coordinated with a release PR merge.** The fix commit itself is `fix:`, so the v5.2.3 release PR (PR #88) will pick it up once PR #86 + this PR merge to main. The Open step's `--force-push` will then correctly rebase the v5.2.3 release branch onto the new main HEAD when the v5.2.3 release PR is finalized. No extra coordination needed beyond the normal PR-merge flow.
