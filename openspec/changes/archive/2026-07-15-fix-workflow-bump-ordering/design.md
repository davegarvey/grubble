## Context

The `version.yml` workflow currently runs steps in this order: `Detect merged release PR` → `Bump (dry-run)` → `Release merged PR` → `Open or update release PR`. When a release PR auto-merges, the `Release` step creates a git tag via the GitHub API. But the `Bump` step already ran before the tag existed, so it sees a stale last-tag (e.g., v5.3.2 instead of freshly-created v5.5.0) and re-analyzes already-released commits.

This happened in practice: after release v5.5.0 was created, the next run computed v5.6.0 (redundantly) because the dry-run still saw v5.3.2 as the latest tag.

Additionally, the macOS build runner had a transient disk I/O failure during `actions-rust-lang/setup-rust-toolchain`. No retry mechanism exists to self-heal.

## Goals / Non-Goals

**Goals:**
- Eliminate the redundant version bump after release PR merge
- Ensure the dry-run always sees the latest tag
- Self-heal transient runner failures

**Non-Goals:**
- Changing the release-please-style workflow semantics (tag-on-merge, auto-merge with PAT)
- Adding new capabilities to grubble

## Decisions

1. **Reorder steps to Release → Bump**: The canonical release-please flow is: (1) detect merged release PR, (2) create tag, (3) compute next version. Moving `Bump` after `Release` ensures the dry-run sees the tag just created. The previous ordering (Bump before Release) was an oversight from the original implementation.

2. **git fetch --tags before Bump**: The `Release` step creates tags via the GitHub API, not through git. The local clone has no knowledge of the newly-created tag. Adding `git fetch origin --tags --force` before the dry-run bridges this gap. Also kept in the `Open` step as defence-in-depth for CHANGELOG generation.

3. **Retry logic for build steps**: Wrapping `cargo build` in a 3-attempt bash loop handles transient compiler/runtime failures. Adding a fallback `Setup Rust (retry)` step that uses `rustup` directly handles the (rare) case where `actions-rust-lang/setup-rust-toolchain` fails due to runner I/O errors.

4. **Stale cleanup still before Release**: The "Clean up stale tags" step must run before any tag-dependent step. It remains at its current position (before Detect), which is correct.

## Risks / Trade-offs

- The `Bump` step now runs even when no merged release PR was found. This is identical behaviour to before (Bump always ran) — just at a different position.
- The `Open` step still fetches tags (defence-in-depth). This adds ~2s but is harmless. Removing it would be an optimisation for a future change.
- The retry fallback installs Rust via `rustup.rs` which lacks the caching provided by `actions-rust-lang/setup-rust-toolchain`. On retry, subsequent `cargo build` will be slower due to cold cache.
