## ADDED Requirements

### Requirement: Create PR after push

The system SHALL allow users to automatically create a pull request from the branch to the default branch after pushing.

#### Scenario: Create PR with branch set

- **WHEN** `create-pr` is true and `branch` is set to "release/v0.35.0"
- **THEN** after pushing, the action SHALL run `gh pr create --base <default-branch> --head release/v0.35.0 --title "Release v<version>" --body "..."`

#### Scenario: Create PR without branch fails

- **WHEN** `create-pr` is true and `branch` is not set
- **THEN** the action SHALL fail with a clear error: "create-pr requires branch to be set"

### Requirement: PR title and body

The PR title and body SHALL be auto-generated with the version information.

#### Scenario: PR title contains version

- **WHEN** a PR is created
- **THEN** the PR title SHALL be "Release v<version>" where <version> is the bumped version

#### Scenario: PR body contains version details

- **WHEN** a PR is created
- **THEN** the PR body SHALL include the new version, previous version, and bump type

### Requirement: Auto-merge support

The system SHALL allow users to enable auto-merge on the created PR.

#### Scenario: Auto-merge with squash

- **WHEN** `auto-merge` is true and `create-pr` is true
- **THEN** after creating the PR, the action SHALL run `gh pr merge --auto --squash`

#### Scenario: Auto-merge without create-pr fails

- **WHEN** `auto-merge` is true and `create-pr` is not true
- **THEN** the action SHALL fail with a clear error: "auto-merge requires create-pr to be set"

### Requirement: Permissions documentation

The system SHALL document the minimum required GitHub token permissions.

#### Scenario: Documented permissions

- **WHEN** a user reads the README
- **THEN** they SHALL see the minimum permissions: `contents: write` and `pull-requests: write`
