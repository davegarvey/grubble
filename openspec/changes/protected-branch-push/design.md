## Context

Grubble is a Rust binary distributed as a GitHub composite action. All git operations (commit, tag, push) are handled by `src/git.rs` and called from `src/main.rs`. The `action.yml` wraps the binary with shell steps for OS detection, binary download, and input mapping.

The core problem: `GITHUB_TOKEN` cannot bypass branch protection rules. The current `push: true` calls `git push` to HEAD, which fails on protected branches.

The solution has three parts:
1. **`branch-push`** — modify the Rust binary to accept a `--git-branch` arg and push to that branch
2. **`pr-creation`** — add action.yml shell steps using `gh` CLI to create PRs
3. **`token-auth`** — add action.yml shell steps with `git remote set-url` for custom tokens

Part 1 requires Rust changes. Parts 2 and 3 are pure action.yml changes.

## Goals / Non-Goals

**Goals:**
- Allow users with protected branches to use grubble's push functionality
- Follow industry best practice (PR-based release workflow)
- Preserve full backward compatibility
- Keep the token input as a secure escape hatch

**Non-Goals:**
- Changing the default behavior of `push: true` (breaking change — deferred to v6)
- Auto-generating branch names when not provided (explicit > implicit)
- Adding a `git-url` input for fork/monorepo target remotes
- Adding a `merge-method` input (squash is the right default for release PRs)

## Decisions

### Decision 1: Branch push in Rust vs action.yml

**Chosen: Rust binary handles branch push**

The branch argument is intrinsically part of the git push operation. The Rust binary already calls `git::push()` — adding a branch parameter is a small, clean change. Alternatives considered:
- **Action.yml shell**: Create the branch and `git push origin <branch>` in shell before/after the binary. This would mean the binary still pushes to HEAD, then the action re-pushes to the branch. Duplicated push effort, confusing.
- **Action.yml + env var**: Set an env var `GIT_BRANCH` for the binary. More indirect than a CLI arg. Prefer explicit CLI args.

### Decision 2: PR creation in Rust vs action.yml

**Chosen: Action.yml shell steps using `gh` CLI**

The `gh` CLI is pre-installed on GitHub Actions runners and handles all the edge cases (PR creation, auto-merge, idempotency). Reimplementing this in Rust would require:
- Adding a GitHub API client dependency
- Handling OAuth token management
- More complex error handling
- No benefit over `gh`

### Decision 3: Token handling in Rust vs action.yml

**Chosen: Action.yml shell steps with `::add-mask::`**

The token must never appear in process lists, logs, or command arguments. By handling it in action.yml:
- `::add-mask::` prevents log leakage
- `git remote set-url` passes the token via git's credential mechanism, not CLI args
- The Rust binary never sees the token

### Decision 4: Validation in Rust vs action.yml

**Chosen: Validation in action.yml for PR/token inputs, Rust validates branch**

The `branch` input is passed to the Rust binary as `--git-branch` — the binary only receives it if set. The action.yml validates cross-input constraints:
- `create-pr: true` without `branch` → fail early
- `auto-merge: true` without `create-pr: true` → fail early

### Decision 5: Push behavior with `branch` set

**Chosen: `git push --set-upstream origin <branch>` instead of `git push origin <branch>`**

Setting upstream is standard practice: it establishes the tracking relationship so subsequent `git push` without args works. The branch is created locally via `git checkout -b` before the commit.

### Decision 6: Default merge method for auto-merge

**Chosen: Squash merge (`gh pr merge --auto --squash`)**

Squash is the standard for release PRs in the industry (release-please uses it). It keeps the default branch history clean with a single "release" commit. Merge commits would preserve the full branch history, which is noisy. Rebase would rewrite history unnecessarily.

## Risks / Trade-offs

- **[Token security] → Mitigation**: `::add-mask::` and never pass token to binary. Token is only used in `git remote set-url` which is git's standard credential mechanism.
- **[Branch already exists] → Mitigation**: If the branch already exists (e.g., from a previous run), `git push --set-upstream` will push new commits to it. The `create-pr` flow may create duplicate PRs — users should ensure unique branch names per version or delete old branches.
- **[`gh` CLI version differences] → Mitigation**: GitHub-hosted runners always have the latest `gh`. Self-hosted runners may need `gh` installed — documented in README.
- **[Breaking change risk] → Mitigation**: All new inputs default to their existing behavior. `branch: ""` preserves `push to HEAD`. Backward compatible for all existing users.

## Open Questions

- Should `create-pr` with an existing PR delete/recreate it, or push new commits to the existing PR? release-please updates existing PRs — that's more complex and could be a follow-up.
- Should we add `gh auth status` check before running `gh pr create` to fail early if `gh` is not authenticated? Low risk on GitHub-hosted runners where it's pre-authenticated.
