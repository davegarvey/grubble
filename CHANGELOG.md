# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.9.4] - 2026-07-09

### Changed

- Merge pull request #55 from davegarvey/fix/action-output-corruption

### Fixed

- prevent double-capture in --raw fallback corrupting GITHUB_OUTPUT

## [4.9.3] - 2026-07-07

### Fixed

- monorepo/npm workspace version readback in action outputs

## [4.9.2] - 2026-06-29

### Fixed

- handle grubble exit 1 as clean no-op, not error
- handle grubble exit 1 as clean no-op, not error

## [4.9.1] - 2026-06-29

### Fixed

- preset-aware version detection in composite action
- use preset-aware version detection in composite action

## [4.9.0] - 2026-04-15

### Added

- add --bump-type and --dry-run flags for CI/CD integration

### Changed

- Merge pull request #48 from davegarvey/feat/bump-type-and-dry-run

## [4.8.0] - 2025-12-25

### Added

- set default git user name and email in GitHub Action

### Changed

- Merge pull request #45 from davegarvey:git-id

### Fixed

- always set git user config when provided

## [4.7.3] - 2025-12-17

### Changed

- Merge pull request #44 from davegarvey:fix-sha

### Fixed

- correct checksum verification logic in GitHub Action

## [4.7.2] - 2025-12-16

### Changed

- Merge pull request #43 from davegarvey:md-fix
- improve CONTRIBUTING.md formatting
- add contributing guide and development tools
- improve markdown linter test behavior

### Fixed

- correct changelog formatting and spacing between releases

## [4.7.1] - 2025-12-16

### Changed

- Merge pull request #42 from davegarvey:md-fix
- add markdownlint configuration for changelog compliance

### Fixed

- correct changelog formatting for markdownlint compliance

## [4.7.0] - 2025-12-16

### Added

- add markdown compliance to changelog generation

### Changed

- Merge pull request #41 from davegarvey:md-lint
- fix typo in sc.prompt.md
- fix CHANGELOG.md formatting
- add markdownlint-cli to CI workflow

### Fixed

- correct markdown formatting in changelog generation

## [4.6.0] - 2025-12-16

### Added

- add changelog generation feature

### Changed

- Merge pull request #40 from davegarvey:changelog
- enable changelog generation in project workflow
- Merge pull request #39 from davegarvey:verbosity
- reduce GitHub Action output verbosity
- Merge pull request #38 from davegarvey:bumper-to-grubble
- update bumper references to grubble
