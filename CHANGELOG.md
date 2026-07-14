# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [5.2.4] - 2026-07-14

### Changed

- restructure README into dedicated docs/ pages (#95)
- track openspec artifacts (#93)

### Fixed

- also unset git extraheader when using PAT for push (#101)
- use PAT for release branch push so CI triggers on release PRs (#100)
- update release PR body on force-push so CHANGELOG entry stays current (#99)
- get_last_tag should exclude floating major/minor tags (v5, v4) (#98)
- remove tag/releaseNotes/updateMajorTag from .versionrc.json to prevent Open step tag-push failure (#96)
- remove bubble dependency, use CHANGELOG entry as release body
- respect user's version pin when downloading binary (#91) (#92)

## [5.2.3] - 2026-07-14

### Changed

- present the canonical release-please flow as the recommended path (#85)
- archive canonical-semver-workflow + sync delta specs (#84)
- switch version.yml to canonical release-please flow (#80)

### Fixed

- decouple Open and Release steps so the workflow handles quiet periods (#86)
- use -F for force=true to send as boolean (#83)
- resolve orphan tags from old flow + fix force=true bug (#82)
- use --raw --dry-run to capture the next version cleanly (#81)

## [5.2.2] - 2026-07-14

### Changed

- Merge pull request #78 from davegarvey/fix/version-merge-commit
- Merge remote-tracking branch 'origin/main' into release/v5.2.1

### Fixed

- use merge commit so release tag stays reachable

## [5.2.1] - 2026-07-14

### Fixed

- exclude floating major/minor tags from version detection (#75)
- create-pr + auto-merge flow in action.yml (#73) (#74)

## [5.2.0] - 2026-07-13

### Added

- protected branch push support (--git-branch, create-pr, auto-merge, token) (#61)

### Changed

- trigger version workflow (round 2) (#71)
- trigger version workflow (#69)
- consolidate bump steps and add stale tag cleanup (#67)
- use published action (davegarvey/grubble@v5) instead of manual build (#60)

### Fixed

- only delete unreachable tags and fetch tags explicitly (#68)
- set GH_TOKEN for gh CLI in push/PR step (#66)
- grep Cargo.toml directly for version instead of --raw (#65)
- move step condition into shell script (#64)
- remove push from .versionrc.json to avoid unwanted pushes (#63)
- push to release branch instead of protected main (#62)

## [5.1.0] - 2026-07-13

### Added

- add protected branch support — branch, create-pr, auto-merge, token inputs

### Changed

- Revert "feat: add protected branch support — branch, create-pr, auto-merge, token inputs"

## [5.0.1] - 2026-07-09

### Fixed

- grubble fails when package file version is ahead of latest tag

## [5.0.0] - 2026-07-09

### Changed

- revert Cargo.toml to 4.9.4 to let version.yml drive the v5.0.0 bump
- **BREAKING:** v5.0.0 — fix exit-code contract, honor --raw --preset, add JSON output
- track opencode project configuration
- restructure README and fix CONTRIBUTING

## [5.0.0] - Unreleased

### Breaking

- Exit code is now 0 for any successful run, including clean no-ops. Previously, exit 1 was used to signal "no bump needed"; exit 1 now means "error" only. Scripts that gate on the exit code of `grubble` or `grubble --dry-run` must switch to `grubble --bump-type` and check its stdout.
- `grubble --raw` now honors `--preset`. `--raw --preset rust` reads from `Cargo.toml`; `--raw --preset node` reads from `package.json`; `--raw --preset git` reads from the latest tag. Previously, `--raw` always read from git tags.

### Added

- New `--output text|json` flag for `--bump-type` and `--raw`. Emits a stable JSON schema suitable for parsing from CI scripts. Rejected when combined with the normal run mode or `--dry-run`.
- New `output` config field (CLI-only, not loaded from `.versionrc.json`).

### Changed

- GitHub Action's "Get current version" step uses a single `./grubble --raw --preset <preset>` invocation, removing preset-specific shell branches. The `set +e` + exit-code branching workaround around the bump step is removed.
- GitHub Action requires `.sha256` checksums for release assets. A missing checksum is now a hard error (`::error::` + `exit 1`), previously a warning.
- `--dry-run` and `--raw` always exit 0 when they complete successfully. Use `grubble --bump-type` for the "would a bump happen?" signal.

### Removed

- Stale `scripts/validate-release.sh` (referred to a non-existent `release.yml` and the old `bumper` binary).
- The `|| true` / `set +e` / `GRUBBLE_EXIT` shell workarounds in `action.yml` and `version.yml` are no longer needed.

### Notes

- The `@v4` floating major tag and all `@v4.x.x` specific tags remain available indefinitely. The default floating tag shifts to `@v5` once v5 ships. Pin to `@v4` or `@v4.9.4` to stay on the v4 contract.
- The in-flight OpenSpec change `fix-action-version-detection-and-output-mapping` is superseded by this release (the underlying problem is now fixed in `--raw`, not patched in the action shell).

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
