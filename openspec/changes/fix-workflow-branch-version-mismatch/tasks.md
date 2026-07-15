## 1. Restructure Open Step

- [x] 1.1 Prepend `git fetch origin --tags --force` before the grubble invocation in the Open step
- [x] 1.2 Create a temporary branch (`release/_tmp`) instead of `release/v<VERSION>` from the dry-run
- [x] 1.3 Run grubble without `--push --force-push --git-branch` (just bump + commit + CHANGELOG)
- [x] 1.4 Read actual version from Cargo.toml after grubble completes (with error handling for empty result)
- [x] 1.5 Check if actual version equals latest tag — if so, delete temp branch and exit (no bump needed)
- [x] 1.6 Detect stale release branches/PRs for the dry-run version — close PR and delete branch if version diverged
- [x] 1.7 Rename temp branch to `release/v<actual_version>` and push with `--force-with-lease`
- [x] 1.8 Use actual version for PR title (`Release v<actual_version>`) and body (CHANGELOG entry)

## 2. Remove Dry-Run Version from Branch Naming

- [x] 2.1 Remove `steps.bump.outputs.version` as the source of truth for the branch name
- [x] 2.2 Ensure all references to `$VERSION` and `$BRANCH` in the Open step derive from the actual version, not the dry-run
- [x] 2.3 Update inline comments to explain the new flow (branch name follows file content, not dry-run)

## 3. Spec Updates

- [x] 3.1 Update `openspec/specs/canonical-release-workflow/spec.md` with the modified requirements from this change
- [ ] 3.2 Archive this change once implementation is verified
