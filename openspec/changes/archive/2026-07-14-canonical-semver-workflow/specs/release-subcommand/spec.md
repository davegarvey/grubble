# release-subcommand Specification

## Purpose
Defines the `grubble --release-from-pr <NUMBER>` flag: resolves a merged release PR to a tag specification (version, tag name, major tag name, merge commit SHA, PR title and body) via the GitHub API. Used by the version workflow post-merge to create the tag and GitHub Release on the main merge commit.
## Requirements
### Requirement: grubble --release-from-pr resolves a merged release PR to a tag spec
The `grubble --release-from-pr <NUMBER>` flag SHALL resolve the given merged release PR to a tag specification. The output SHALL be a JSON object (when `--output json` is passed) or human-readable text (default) containing:
- `version`: the version of the release (e.g., `5.2.1`)
- `tag_name`: the git tag name (e.g., `v5.2.1`)
- `major_tag_name`: the floating major tag name (e.g., `v5`)
- `merge_commit_sha`: the SHA of the merge commit on main
- `title`: the PR title
- `body`: the PR body (used for the GitHub Release description)

The flag SHALL exit 0 on success and non-zero if the PR is not found, not merged, or the head branch does not match the `release/v<version>` pattern. The flag is mutually exclusive with the bump modes (e.g., `--push`, `--tag`, `--changelog`) because it is a read-only resolution operation.

#### Scenario: grubble --release-from-pr 79 --output json
- **WHEN** the user runs `grubble --release-from-pr 79 --output json` and PR #79 is a merged release PR with head branch `release/v5.2.2` and merge commit `5826431`
- **THEN** the command SHALL print a JSON object `{"version":"5.2.2","tag_name":"v5.2.2","major_tag_name":"v5","merge_commit_sha":"5826431...","title":"Release v5.2.2","body":"..."}` and exit 0

#### Scenario: PR is not merged
- **WHEN** the user runs `grubble --release-from-pr <num>` and the PR is open (not merged)
- **THEN** the command SHALL print an error to stderr and exit non-zero

#### Scenario: PR head branch does not match release pattern
- **WHEN** the user runs `grubble --release-from-pr <num>` and the PR's head branch is `feature-x` (not `release/v*`)
- **THEN** the command SHALL print an error to stderr and exit non-zero

#### Scenario: PR does not exist
- **WHEN** the user runs `grubble --release-from-pr 99999` and no PR with that number exists
- **THEN** the command SHALL print an error to stderr and exit non-zero

### Requirement: grubble --release-from-pr authenticates with the GitHub API
The `grubble --release-from-pr` flag SHALL authenticate with the GitHub API. The authentication method SHALL respect the `GH_TOKEN` or `GITHUB_TOKEN` environment variables. If neither is set, the command SHALL print an error and exit non-zero.

#### Scenario: GH_TOKEN is set
- **WHEN** the `GH_TOKEN` environment variable is set to a valid GitHub token
- **THEN** the flag SHALL use it to authenticate against the GitHub API

#### Scenario: no token is set
- **WHEN** neither `GH_TOKEN` nor `GITHUB_TOKEN` is set
- **THEN** the command SHALL print an error to stderr explaining that a GitHub token is required and exit non-zero

### Requirement: grubble --release-from-pr is documented in --help and README
The `grubble --release-from-pr` flag SHALL appear in `grubble --help` output with a one-line description. The README's CLI section SHALL list the flag with a brief explanation and an example.

#### Scenario: grubble --help lists the release-from-pr flag
- **WHEN** the user runs `grubble --help`
- **THEN** the output SHALL include a `--release-from-pr <NUMBER>` flag entry with a description like "Resolve a merged release PR to a tag spec (used by the version workflow post-merge)"
