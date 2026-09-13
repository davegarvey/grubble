# Agent instructions

## Worktree workflow

Keep the primary checkout on `main` and use a dedicated Git worktree for feature, fix, documentation, and other non-main work. Keep worktree directories under the repository's ignored `.worktrees/` directory.

### Before starting work

Run these commands from the primary checkout. Use a conventional branch name such as `feat/example` and a separate worktree directory name without `/` characters:
- `git fetch origin`
- `mkdir -p .worktrees`
- `git worktree add -b <branch-name> .worktrees/<worktree-name> origin/main`
- `cd .worktrees/<worktree-name>`

For example:

```sh
git fetch origin
mkdir -p .worktrees
git worktree add -b feat/example .worktrees/feat-example origin/main
cd .worktrees/feat-example
```

Do all implementation and commits from the feature worktree; do not switch the primary checkout to the feature branch. Use `git worktree list` from the primary checkout to inspect active worktrees. A branch can only be checked out in one worktree at a time.

### After completing a PR

From the primary checkout, not from inside the worktree, remove the worktree and then its local branch:

1. `git worktree remove .worktrees/<worktree-name>`
2. `git branch -d <branch-name>`
3. `git fetch origin --prune`

Use `git branch -D <branch-name>` only after confirming a squash-merged or otherwise unrecognized branch is already merged. Do not manually delete worktree directories or use `--force` when removing one unless intentionally discarding its uncommitted changes. If a worktree was deleted manually, run `git worktree prune` to remove stale metadata.
