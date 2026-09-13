# action-create-pr-flow Specification

## Purpose

Defines how the composite GitHub Action coordinates version bumps, release-branch pushes, pull-request creation, and optional auto-merge behavior.

## Requirements

### Requirement: Run bump declares the branch as a step output
The "Run bump" step in `action.yml` SHALL write a `branch=...` line to its `$GITHUB_OUTPUT` so the composite action's declared `branch` output resolves to a real value. The value SHALL be `inputs.branch` when set, `release/v<new_version>` when `inputs.create-pr` is true and the new version differs from the previous version, and empty otherwise.

#### Scenario: explicit branch input
- **WHEN** `inputs.branch` is set to `release/v0.35.0`
- **THEN** the `branch` step output of "Run bump" SHALL be `release/v0.35.0`, regardless of whether a version bump occurred

#### Scenario: auto-generated branch on bump
- **WHEN** `inputs.create-pr` is `true`, `inputs.branch` is empty, and a version bump occurred (new version differs from previous)
- **THEN** the `branch` step output of "Run bump" SHALL be `release/v<new_version>`

#### Scenario: no branch when no bump and no explicit branch
- **WHEN** `inputs.create-pr` is `true`, `inputs.branch` is empty, and no version bump occurred (new version equals previous)
- **THEN** the `branch` step output of "Run bump" SHALL be empty

#### Scenario: no branch when create-pr is false
- **WHEN** `inputs.create-pr` is `false` (or not set) and `inputs.branch` is empty
- **THEN** the `branch` step output of "Run bump" SHALL be empty

### Requirement: Auto-generated release branch is pushed before PR creation
The composite action SHALL contain a step that pushes the auto-generated `release/v<version>` branch to the remote before any step invokes `gh pr create`. The push step SHALL run only when a bump occurred, `inputs.create-pr` is `true`, `inputs.branch` is empty, and the generated branch is non-empty. It SHALL recreate the local branch and force-push it and its tags. The push step SHALL NOT run when `inputs.branch` is set, because grubble's named-branch flow already pushed that branch.

#### Scenario: auto-generated branch is pushed when no explicit branch
- **WHEN** `inputs.create-pr` is `true`, `inputs.branch` is empty, and a version bump occurred
- **THEN** a step SHALL push the branch `release/v<new_version>` to the remote before `gh pr create` is invoked

#### Scenario: explicit branch is not re-pushed by the action
- **WHEN** `inputs.create-pr` is `true` and `inputs.branch` is set to `release`
- **THEN** the action SHALL NOT run a `git push` for the branch

#### Scenario: stale remote branch is overwritten
- **WHEN** a previous run left `release/v0.35.0` on the remote and a new run produces the same version
- **THEN** the push step SHALL recreate the local branch and force-push it so the update succeeds instead of failing with a non-fast-forward error

### Requirement: Create release PR step authenticates with GH_TOKEN
The "Create release PR" step in `action.yml` SHALL set `GH_TOKEN: ${{ github.token }}` (or `GITHUB_TOKEN: ${{ github.token }}`) in its `env:` block, so that `gh pr create` succeeds in the composite-action context.

#### Scenario: gh pr create has auth
- **WHEN** the "Create release PR" step runs
- **THEN** `$GH_TOKEN` SHALL be set in the step environment to `${{ github.token }}` before `gh pr create` is invoked

### Requirement: Create release PR step uses the branch from step outputs
The "Create release PR" step SHALL source its head branch from `${{ steps.bump.outputs.branch }}`, which is populated by the "Run bump" step. The step SHALL NOT reference local shell variables from the "Run bump" step's `run:` block.

#### Scenario: --head is non-empty
- **WHEN** the "Create release PR" step runs and `steps.bump.outputs.branch` is `release/v0.35.0`
- **THEN** `gh pr create --head release/v0.35.0 ...` SHALL be invoked

### Requirement: Enable auto-merge only enables auto-merge
The "Enable auto-merge" step SHALL invoke `gh pr merge --auto --squash` on the URL emitted by the "Create release PR" step's `pr_url` output. The step SHALL NOT contain branch-creation or `git push` logic, and SHALL NOT reference local shell variables from a different `run:` block.

#### Scenario: auto-merge runs after PR is created
- **WHEN** `inputs.auto-merge` is `true`, `inputs.create-pr` is `true`, and a version bump occurred
- **THEN** the "Enable auto-merge" step SHALL run `gh pr merge "${{ steps.pr.outputs.pr_url }}" --auto --squash`

#### Scenario: auto-merge skips when no PR URL
- **WHEN** `steps.pr.outputs.pr_url` is empty
- **THEN** the "Enable auto-merge" step SHALL skip the `gh pr merge` call without error

### Requirement: Step ordering is bump, push, PR, auto-merge
The composite action SHALL execute its PR-flow steps in this order, with each step conditioned to skip when its work is not needed: Run bump, Push release branch when auto-generated, Create release PR, and Enable auto-merge.

#### Scenario: auto-generated branch path ordering
- **WHEN** `inputs.create-pr` is `true`, `inputs.branch` is empty, and a version bump occurred
- **THEN** "Push release branch" SHALL run after "Run bump" and before "Create release PR"

#### Scenario: explicit branch path ordering
- **WHEN** `inputs.create-pr` is `true` and `inputs.branch` is set
- **THEN** "Push release branch" SHALL be skipped, and "Create release PR" SHALL run after "Run bump"

#### Scenario: no-PR path ordering
- **WHEN** `inputs.create-pr` is `false` (or not set)
- **THEN** "Push release branch", "Create release PR", and "Enable auto-merge" SHALL all be skipped
