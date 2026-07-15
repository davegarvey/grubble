## 1. Grubble CLI Changes

- [x] 1.1 Remove the `--output json` restriction that limits it to `--bump-type`, `--raw`, or `--release-from-pr` (line 180-184 in main.rs)
- [x] 1.2 Route `log()` messages to stderr when `--output json` is active, keeping stdout clean for the JSON payload
- [x] 1.3 Emit `{"version": "x.y.z"}` JSON output at the end of the bump path (after commit/push)
- [x] 1.4 Build and sanity-check: `cargo build --release`

## 2. Workflow Changes

- [x] 2.1 In `version.yml`, replace the grep-based version extraction with parsing grubble's `--output json` via `jq`
- [x] 2.2 Add `--output json` to the grubble invocation in the Open step

## 3. Spec Updates

- [ ] 3.1 Update `openspec/specs/canonical-release-workflow/spec.md` with the modified version extraction requirement
- [ ] 3.2 Archive this change once implementation is verified
