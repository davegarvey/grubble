## 1. Add --release-version to grubble CLI

- [ ] 1.1 Add `--release-version <VERSION>` arg to `Args` struct with `conflicts_with_all`
- [ ] 1.2 Implement handler: parse version, write via strategy, optional changelog, commit
- [ ] 1.3 Wire up JSON output support
- [ ] 1.4 Run existing tests to confirm no regressions

## 2. Update version.yml Open step

- [ ] 2.1 Replace forward-bump call with `grubble --release-version $VERSION --changelog --output json`
- [ ] 2.2 Remove stale branch/PR cleanup (no longer needed since versions don't diverge)
- [ ] 2.3 Simplify branch naming to use `steps.bump.outputs.version` directly

## 3. Update spec docs

- [ ] 3.1 Sync delta specs to main specs
- [ ] 3.2 Update docs/release-workflow.md if needed

## 4. PR and merge

- [ ] 4.1 Create branch from origin/main
- [ ] 4.2 Commit changes, push, create PR
- [ ] 4.3 Merge PR

## 5. Verify

- [ ] 5.1 Confirm workflow ran on merged commit
- [ ] 5.2 Confirm release PR was created correctly with matching version
- [ ] 5.3 Confirm no file-ahead-of-tag errors
