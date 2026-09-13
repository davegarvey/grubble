## Resolution

This change is superseded by `fix-release-workflow-redesign`. PR #118 shipped the temporary-branch implementation, but PR #122 replaced it with `--release-version`, which intentionally uses the dry-run version directly and removes the sync-divergence path. No delta from this change should be synced or applied.
