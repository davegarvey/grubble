# changelog-entry-flag Specification

## Purpose
Defines the `grubble --changelog-entry` CLI flag: reads the local `CHANGELOG.md` file and prints the most recent (first) release entry. Used by the release workflow to set the PR body to the changelog content, which then flows to the GitHub Release body after merge.

## Requirements

### Requirement: --changelog-entry prints the latest changelog entry
The `grubble --changelog-entry` flag SHALL read `CHANGELOG.md` from the current working directory and print the most recent entry (the first `## [version]` section). The entry SHALL include the version header through all its categorized changes, stopping before the next `## [` header or EOF.

#### Scenario: CHANGELOG.md exists with multiple entries
- **WHEN** the user runs `grubble --changelog-entry` and `CHANGELOG.md` contains entries for 5.2.3 (latest) and 5.2.2 (previous)
- **THEN** the command SHALL print the full 5.2.3 entry including its version header, category headers, and list items, and exit 0

#### Scenario: CHANGELOG.md does not exist
- **WHEN** the user runs `grubble --changelog-entry` and `CHANGELOG.md` does not exist
- **THEN** the command SHALL print nothing and exit 0

#### Scenario: CHANGELOG.md is empty
- **WHEN** the user runs `grubble --changelog-entry` and `CHANGELOG.md` is empty
- **THEN** the command SHALL print nothing and exit 0

#### Scenario: CHANGELOG.md has only the header
- **WHEN** the user runs `grubble --changelog-entry` and `CHANGELOG.md` has a header but no version entries
- **THEN** the command SHALL print nothing and exit 0

### Requirement: --changelog-entry is a read-only flag
The `--changelog-entry` flag SHALL not modify any files, create any commits, or make any network calls. It is a pure read-only extraction from the local `CHANGELOG.md` file.

#### Scenario: side-effect-free
- **WHEN** the user runs `grubble --changelog-entry`
- **THEN** no files SHALL be written, no git commands SHALL be executed, and no network calls SHALL be made

### Requirement: --changelog-entry is documented in --help and README
The `grubble --changelog-entry` flag SHALL appear in `grubble --help` output with a one-line description. The README SHALL list the flag in the CLI usage section.

#### Scenario: grubble --help lists the changelog-entry flag
- **WHEN** the user runs `grubble --help`
- **THEN** the output SHALL include a `--changelog-entry` flag entry

#### Scenario: README includes --changelog-entry
- **WHEN** the user reads the README CLI usage section
- **THEN** the section SHALL list `--changelog-entry` with a brief description
