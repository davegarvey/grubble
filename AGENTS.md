# Agent instructions

## After completing a PR

1. Move back to main: `git checkout main`
2. Reset to origin/main (local main is often stale): `git reset --hard origin/main`
3. Delete the local branch (use `-D` for squash-merged branches): `git branch -d <branch-name>`
4. Prune stale remote tracking refs: `git fetch origin --prune`
