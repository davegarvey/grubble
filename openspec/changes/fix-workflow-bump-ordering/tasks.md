## 1. Workflow Step Reordering

- [ ] 1.1 Reorder workflow steps: move `Bump (dry-run)` after `Release merged PR` in version.yml
- [ ] 1.2 Add `git fetch origin --tags --force` before the dry-run in the Bump step
- [ ] 1.3 Update step comments and flow docstring to reflect new ordering

## 2. Build Retry Logic

- [ ] 2.1 Wrap `cargo build --release` in a 3-attempt retry loop
- [ ] 2.2 Add `Setup Rust (retry)` fallback step that uses direct `rustup` install

## 3. Spec Update

- [ ] 3.1 Update `canonical-release-workflow` delta spec with MODIFIED step ordering requirement, Bump fetches tags requirement, and REMOVED Open-fetches-tags requirement

## 4. Verification

- [ ] 4.1 Run all tests: `cargo test --all-features`
- [ ] 4.2 Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 4.3 Run fmt: `cargo fmt --check`
- [ ] 4.4 Create PR, merge, verify post-merge workflow run completes without redundant bump
