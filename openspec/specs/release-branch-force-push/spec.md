# release-branch-force-push Specification

## Purpose
Defines the `grubble --force-push` safety contract for synchronizing workflow-owned release branches without silently overwriting concurrent remote updates.

## Requirements

### Requirement: grubble --force-push requires --git-branch
The `grubble --force-push` flag SHALL only be accepted in combination with `--git-branch <NAME>`. If `--force-push` is set without `--git-branch`, the binary SHALL exit with a non-zero code and print a clear error to stderr explaining that force-push is only meaningful for a specific named branch. Force-pushing to the current branch (`main`, etc.) is out of scope and is rejected to keep the safety guarantees of the regular `--push` flow intact.

#### Scenario: --force-push without --git-branch
- **WHEN** the user runs `grubble --force-push --push` (no `--git-branch`)
- **THEN** the command SHALL print an error to stderr explaining that `--force-push` requires `--git-branch` and exit non-zero

#### Scenario: --force-push with --git-branch
- **WHEN** the user runs `grubble --force-push --push --git-branch release/v1.0.0`
- **THEN** the command SHALL accept the flag combination and the push SHALL use `git push --force-with-lease origin release/v1.0.0`

### Requirement: grubble --force-push uses --force-with-lease semantics
When `grubble --force-push` is in effect, the underlying `git push` SHALL use `--force-with-lease` (not plain `--force`). The lease variant protects against the failure mode of plain `--force`: if the remote branch has been updated since the local checkout (e.g., a human pushed a fix, or another workflow run landed), the push SHALL fail rather than silently overwriting the remote. This matches `release-please`'s own use of `--force-with-lease` for the same reason.

#### Scenario: remote has not been updated
- **WHEN** `grubble --force-push --push --git-branch release/v1.0.0` runs and the remote `release/v1.0.0` is at the expected commit (no concurrent updates)
- **THEN** the push SHALL succeed and overwrite the remote

#### Scenario: remote was updated concurrently
- **WHEN** `grubble --force-push --push --git-branch release/v1.0.0` runs and the remote `release/v1.0.0` has been updated since the local checkout (e.g., a human pushed `git commit --amend` to the branch)
- **THEN** the push SHALL fail with a non-zero exit code and a clear error explaining that the remote was updated (the `--force-with-lease` check), and the local branch SHALL be left unchanged

### Requirement: grubble --force-push is documented in --help and README
The `grubble --force-push` flag SHALL appear in `grubble --help` output with a one-line description. The README's Usage documentation SHALL list the flag with a brief explanation and an example showing it used in combination with `--push --git-branch`. The Best Practices section SHALL have a "Releasing" subsection that explains the release-branch force-push pattern and references `release-please` and `semantic-release` for context.

#### Scenario: grubble --help lists --force-push
- **WHEN** the user runs `grubble --help`
- **THEN** the output SHALL include a `--force-push` flag entry with a description that mentions it requires `--git-branch` and uses `--force-with-lease`

#### Scenario: README documents the release-branch force-push pattern
- **WHEN** the user reads the README's Best Practices > Releasing section
- **THEN** the section SHALL explain why force-push is the right primitive for workflow-owned release branches, why `--force-with-lease` is preferred over plain `--force`, and link to `release-please` and `semantic-release` for the canonical patterns
