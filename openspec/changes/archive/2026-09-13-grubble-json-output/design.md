## Context

Grubble currently supports `--output json` in three modes: `--bump-type`, `--raw`, and `--release-from-pr`. The normal bump path (file writes, commit, push) does not support `--output json` — a guard at `src/main.rs:180-184` explicitly rejects it. After the bump path completes, the only indication of what version was written is the change to Cargo.toml itself, which the workflow reads via `grep`. This is fragile and roundabout.

The canonical release-please pattern favors machine-parseable output from tools so CI/CD pipelines can consume results directly without scraping files.

## Goals / Non-Goals

**Goals:**
- Grubble emits the version as JSON after a bump when `--output json` is requested
- Stdout contains only the JSON payload when `--output json` is active (log messages go to stderr)
- Workflow replaces grep-based version extraction with JSON parsing

**Non-Goals:**
- Changing the JSON format emitted by existing modes (`--raw --output json`, `--bump-type --output json`, `--release-from-pr --output json`)
- Adding new CLI flags

## Decisions

### Decision 1: Route log messages to stderr when `--output json` is active

The `log()` function currently writes to stdout. When `--output json` is requested, stdout must contain only the JSON payload. Rather than threading a new flag through every call site, `log()` checks the output mode. When JSON output is active, `eprintln!` is used instead of `println!`.

**Rationale:** This is the minimum change to keep stdout clean for JSON parsing. GitHub Actions captures both stdout and stderr in the workflow logs, so users still see the informational messages.

**Alternatives considered:**
- *Capture stdout in a buffer and only emit JSON at the end*: More complex, requires changing the `log()` function signature or using a global writer.
- *Suppress log messages entirely when `--output json`*: Loses debugging visibility in CI logs.

### Decision 2: Emit JSON at the end of the bump path, after push

The JSON payload is emitted after files are written, committed, and pushed. This ensures the version is final and all side effects are complete. The payload contains just `{"version": "x.y.z"}` — the workflow only needs the version string.

### Decision 3: Keep existing JSON formats unchanged

The existing `--raw --output json`, `--bump-type --output json`, and `--release-from-pr --output json` paths emit different JSON schemas (they include `preset`, `type`, etc.). These are unchanged. Only the bump path gains JSON output.

## Risks / Trade-offs

- **[Low] `log()` behaviour change**: Changing `log()` to use `eprintln!` when `--output json` is active means informational messages go to stderr. In local terminal use, stderr and stdout both display; no visible difference. In CI, both streams are captured. No practical downside.
- **[Low] Workflow depends on `jq`**: The workflow uses `jq` to parse the JSON. GitHub-hosted runners include `jq` by default. No dependency risk.
