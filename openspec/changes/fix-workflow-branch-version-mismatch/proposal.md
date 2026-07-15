## Why

The "Version & Release" workflow computes the release branch name from a `grubble --raw --dry-run` that skips the "file-behind-tag sync" logic, but the actual `grubble` run in the Open step does not use `--raw`, so the sync logic is active. When Cargo.toml is behind the latest tag (due to prior tag/file mismatches), the sync bumps Cargo.toml to the tag version first, then bumps again based on commit analysis — producing a different version than the dry-run predicted. The branch name (and therefore the tag created by the Release step) no longer matches the actual Cargo.toml content, causing subsequent runs to fail with "package version is ahead of latest tag."

This is a structural bug that will recur whenever tags and file versions drift. It was triggered in practice by an orphaned `v5.4.0` tag placed on the wrong commit, which cascaded into the `v5.3.2` release failure.

## What Changes

- **Workflow (`version.yml`)**: Derive the release branch name from grubble's actual output after it runs (the version written to Cargo.toml), rather than from the pre-computed dry-run. This ensures the branch name always matches the file content.
- **Existing spec update**: Update the `canonical-release-workflow` spec to account for this sync behavior and correct the Bump step's contract.
- **Stale PR/branch cleanup**: When the sync causes the actual version to diverge from the dry-run version, any previously-opened release PR for the dry-run version SHALL be closed to avoid orphaned branches and PRs.

## Capabilities

### New Capabilities

- _None_

### Modified Capabilities

- `canonical-release-workflow`: The `Bump step uses dry-run to compute the next version` requirement currently assumes the dry-run result is authoritative for the branch name. This needs to account for the sync logic: the branch name must be derived after grubble writes the file, and stale branches/PRs from the dry-run version must be cleaned up.

## Impact

- `.github/workflows/version.yml`: Modified — the Open step derives the branch name from the dry-run output; this will change to derive it from grubble's actual write result.
- `openspec/specs/canonical-release-workflow/spec.md`: Updated requirements.
