## ADDED Requirements

### Requirement: Push to configurable branch

The system SHALL allow users to push the version bump commit to a user-specified branch instead of always pushing to HEAD.

#### Scenario: Push to specified branch

- **WHEN** the user sets `branch` input to "release/v0.35.0" and `push` to true
- **THEN** the bump commit SHALL be pushed to `origin/release/v0.35.0`

#### Scenario: Default behavior preserved

- **WHEN** the user sets `push` to true and does NOT set `branch`
- **THEN** the bump commit SHALL be pushed to HEAD (existing behavior)

#### Scenario: Branch does not exist locally

- **WHEN** the user sets `branch` to a branch that does not exist locally
- **THEN** the action SHALL create the branch locally before pushing

#### Scenario: Remote tracking is set

- **WHEN** pushing to a branch
- **THEN** the remote tracking relationship SHALL be set (`git push --set-upstream origin <branch>`)

### Requirement: Branch argument in CLI

The Rust binary SHALL accept a `--git-branch` CLI argument that controls the target branch for git push.

#### Scenario: CLI arg passed to push

- **WHEN** `--git-branch release/v0.35.0` is passed
- **THEN** `git::push("release/v0.35.0")` SHALL be called instead of `git::push("")`

#### Scenario: CLI arg not passed

- **WHEN** `--git-branch` is not passed (defaults to empty string)
- **THEN** `git::push("")` SHALL be called, preserving existing behavior

### Requirement: Force-tag push respects branch

When force-pushing tags (for update-major-tag / update-minor-tag), the branch parameter SHALL also be respected.

#### Scenario: Force tags with branch

- **WHEN** `--git-branch release/v0.35.0` and `--update-minor-tag` are both passed
- **THEN** `git::push_with_force_tags("release/v0.35.0")` SHALL push to that branch and force-push tags

### Requirement: Error handling

The system SHALL handle git push failures gracefully.

#### Scenario: Push failure with error message

- **WHEN** the push to the specified branch fails
- **THEN** the system SHALL exit with a non-zero code and a descriptive error message
