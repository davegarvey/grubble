## ADDED Requirements

### Requirement: Resolve pinned action version in composite action
The action SHALL resolve the pinned version of grubble when consumed as a composite action from another repository. The resolution SHALL pass `github.action_ref` through an `env:` block to ensure correct evaluation in composite action context.

#### Scenario: Pinned full semver tag resolves correctly
- **WHEN** a consumer workflow uses `uses: davegarvey/grubble@v5.3.0`
- **THEN** the action downloads the binary from the `v5.3.0` release

#### Scenario: Floating major tag resolves to latest matching release
- **WHEN** a consumer workflow uses `uses: davegarvey/grubble@v5`
- **THEN** the action downloads the binary from the latest v5.x.y release that has assets

#### Scenario: Floating minor tag resolves to latest matching release
- **WHEN** a consumer workflow uses `uses: davegarvey/grubble@v5.3`
- **THEN** the action downloads the binary from the latest v5.3.x release that has assets

### Requirement: Authenticated API calls for release resolution
The action SHALL use authenticated GitHub API calls (via `gh api` or `GITHUB_TOKEN`) to query release information, avoiding unauthenticated rate limits.

#### Scenario: Authenticated resolution succeeds
- **WHEN** the action queries the GitHub API for release data
- **THEN** the request SHALL include authentication via the runner's `GITHUB_TOKEN`

### Requirement: Validate release has assets before download
The action SHALL verify that the resolved release version has at least one downloadable binary asset before attempting to download. If the release has no assets, the action SHALL emit an actionable error message and fail.

#### Scenario: Release has assets, download proceeds
- **WHEN** the resolved release has binary assets
- **THEN** the action proceeds to download the matching platform binary

#### Scenario: Release has no assets, action fails with clear error
- **WHEN** the resolved release has no binary assets
- **THEN** the action SHALL print an error message listing the release tag and suggesting the user pin to a version with assets

### Requirement: Graceful failure on missing auth token
If `GITHUB_TOKEN` is not available in the runner environment, the action SHALL fall back to unauthenticated API access with a warning rather than failing silently.

#### Scenario: No GITHUB_TOKEN available
- **WHEN** `GITHUB_TOKEN` is not set in the runner environment
- **THEN** the action SHALL print a warning and attempt unauthenticated API access
- **THEN** the action SHALL NOT fail solely due to missing authentication

### Requirement: Not affect non-composite-action usage
The change to action.yml SHALL NOT affect behavior when the action is consumed in non-composite contexts (e.g., the self-build path in version.yml).

#### Scenario: Self-build path unchanged
- **WHEN** version.yml runs the self-build path (cargo build --release)
- **THEN** the "Resolve release version" step is not used and behavior is unchanged
