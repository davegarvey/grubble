## 1. Fix "Get current version" step

- [x] 1.1 Replace `./grubble --raw` with preset-aware extraction for `preset=node`: `node -p "require('./package.json').version"`
- [x] 1.2 Add preset-aware extraction for `preset=rust`: `grep '^version' Cargo.toml | head -1 | cut -d'"' -f2`
- [x] 1.3 Keep `./grubble --raw` for `preset=git` or no preset (unchanged behavior)
- [x] 1.4 Add fallback to `0.0.0` for all extraction methods on failure

## 2. Fix "Run bump" step output

- [x] 2.1 Add `echo "previous_version=$PREV" >> $GITHUB_OUTPUT` after the bump type determination block in the "Run bump" step

## 3. Verify

- [x] 3.1 Run `actionlint` or manual review on the updated `action.yml` for syntax errors
- [ ] 3.2 Test the action end-to-end on a repo with no tags and `preset: node` — verify version is read correctly, bump succeeds, outputs are set
- [ ] 3.3 Test on a repo with existing tags — verify no regression in version detection
