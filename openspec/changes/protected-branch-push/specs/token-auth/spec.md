## ADDED Requirements

### Requirement: Custom token for git push

The system SHALL allow users to supply a custom token (PAT or GitHub App installation token) for git push authentication, enabling bypass of branch protection rules when needed.

#### Scenario: Token used for remote URL

- **WHEN** `token` is set to a valid GitHub token
- **THEN** the action SHALL set the remote URL to `https://x-access-token:<token>@github.com/<owner>/<repo>` before running grubble

#### Scenario: Default token behavior

- **WHEN** `token` is not set (default)
- **THEN** the existing remote URL SHALL be preserved

### Requirement: Token security

The system SHALL prevent the token from leaking into logs.

#### Scenario: Token masked in logs

- **WHEN** `token` is set
- **THEN** the action SHALL run `::add-mask::` on the token value before any step that might log it

#### Scenario: Token not passed as CLI arg

- **WHEN** `token` is set
- **THEN** the token SHALL NOT be passed as a CLI argument to the grubble binary (to avoid exposure in process lists)

### Requirement: Token is an escape hatch

The token input SHALL be documented as an advanced/escape hatch option, not the primary recommendation.

#### Scenario: Documented as escape hatch

- **WHEN** a user reads the README
- **THEN** the token input SHALL be documented under a "Bypass token (advanced)" section, with a note recommending the PR-based flow instead
