## 1. Add --release-version to grubble CLI

- [x] 1.1 Add `--release-version <VERSION>` arg to `Args` struct with `conflicts_with_all`
- [x] 1.2 Implement handler: parse version, write via strategy, optional changelog, commit
- [x] 1.3 Wire up JSON output support
- [x] 1.4 Run existing tests to confirm no regressions

## 2. Update version.yml Open step

- [x] 2.1 Replace forward-bump call with `grubble --release-version $VERSION --changelog --output json`
- [x] 2.2 Remove stale branch/PR cleanup (no longer needed since versions don't diverge)
- [x] 2.3 Simplify branch naming to use `steps.bump.outputs.version` directly

## 3. Update spec docs

- [x] 3.1 Sync delta specs to main specs
- [x] 3.2 Review `docs/release-workflow.md` and update it where the current workflow behavior requires clarification

## 4. PR and merge

- [x] 4.1 Create branch from origin/main
- [x] 4.2 Commit changes, push, create PR #122
- [x] 4.3 Merge PR #122

## 5. Verify

- [x] 5.1 Confirm workflow ran on merged commit
- [x] 5.2 Confirm release PR was created correctly with matching version
- [x] 5.3 Confirm no file-ahead-of-tag errors
