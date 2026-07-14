# release-branch-force-push Specification

## Purpose
Defines the `grumble --force-push` flag: when set in combination with `--git-branch <NAME>`, the push to the named branch uses `git push --force-with-lease` instead of `git push`. Used by the canonical release-please-style workflow to keep a workflow-owned release branch in sync with `main` across multiple workflow runs. The `--force-with-lease` variant (not plain `--force`) protects against clobbering concurrent updates from a human or another workflow.
## Requirements
### Requirement: grumble --force-push requires --git-branch
The `grumble --force-push` flag SHALL only be accepted in combination with `--git-branch <NAME>`. If `--force-push` is set without `--git-branch`, the binary SHALL exit with a non-zero code and print a clear error to stderr explaining that force-push is only meaningful for a specific named branch. Force-pushing to the current branch (`main`, etc.) is out of scope and is rejected to keep the safety guarantees of the regular `--push` flow intact (matches the `release-please` pattern where the release branch is the only branch ever force-pushed).

#### Scenario: --force-push without --git-branch
- **WHEN** the user runs `grumble --force-push --push` (no `--git-branch`)
- **THEN** the command SHALL print an error to stderr explaining that `--force-push` requires `--git-branch` and exit non-zero

#### Scenario: --force-push with --git-branch
- **WHEN** the user runs `grumble --force-push --push --git-branch release/v1.0.0`
- **THEN** the command SHALL accept the flag combination and the push SHALL use `git push --force-with-lease origin release/v1.0.0`

### Requirement: grumble --force-push uses --force-with-lease semantics
When `grumble --force-push` is in effect, the underlying `git push` SHALL use `--force-with-lease` (not plain `--force`). The lease variant protects against the failure mode of plain `--force`: if the remote branch has been updated since the local checkout (e.g., a human pushed a fix, or another workflow run landed), the push SHALL fail rather than silently overwriting the remote. This matches `release-please`'s own use of `--force-with-lease` for the same reason.

#### Scenario: remote has not been updated
- **WHEN** `grumble --force-push --push --git-branch release/v1.0.0` runs and the remote `release/v1.0.0` is at the expected commit (no concurrent updates)
- **THEN** the push SHALL succeed and overwrite the remote

#### Scenario: remote was updated concurrently
- **WHEN** `grumble --force-push --push --git-branch release/v1.0.0` runs and the remote `release/v1.0.0` has been updated since the local checkout (e.g., a human pushed `git commit --amend` to the branch)
- **THEN** the push SHALL fail with a non-zero exit code and a clear error explaining that the remote was updated (the `--force-with-lease` check), and the local branch SHALL be left unchanged

### Requirement: grumble --force-push is documented in --help and README
The `grumble --force-push` flag SHALL appear in `grumble --help` output with a one-line description. The README's Usage section SHALL list the flag with a brief explanation and an example showing it used in combination with `--push --git-branch`. The Best Practices section SHALL have a "Releasing" subsection that explains the release-branch force-push pattern and references `release-please` and `semantic-release` for context.

#### Scenario: grumble --help lists --force-push
- **WHEN** the user runs `grumble --help`
- **THEN** the output SHALL include a `--force-push` flag entry with a description that mentions it requires `--git-branch` and uses `--force-with-lease`

#### Scenario: README documents the release-branch force-push pattern
- **WHEN** the user reads the README's Best Practices > Releasing section
- **THEN** the section SHALL explain why force-push is the right primitive for workflow-owned release branches, why `--force-with-lease` is preferred over plain `--force`, and link to `release-please` and `semantic-release` for the canonical patterns
