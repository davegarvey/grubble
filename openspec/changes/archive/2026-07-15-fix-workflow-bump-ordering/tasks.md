## 1. Workflow Step Reordering

- [x] 1.1 Reorder workflow steps: move `Bump (dry-run)` after `Release merged PR` in version.yml
- [x] 1.2 Add `git fetch origin --tags --force` before the dry-run in the Bump step
- [x] 1.3 Update step comments and flow docstring to reflect new ordering

## 2. Build Retry Logic

- [x] 2.1 Wrap the native `cargo build --release` in a 3-attempt retry loop
- [x] 2.2 Add `Setup Rust (retry)` fallback step that uses direct `rustup` install

## 3. Spec Update

- [x] 3.1 Update `canonical-release-workflow` delta spec with MODIFIED step ordering requirement, Bump fetches tags requirement, and REMOVED Open-fetches-tags requirement

## 4. Verification

- [x] 4.1 Run all tests: `cargo test --all-features`
- [x] 4.2 Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 4.3 Run fmt: `cargo fmt --check`
- [x] 4.4 PR #125 merged, and subsequent workflow runs/releases completed without the redundant bump.
