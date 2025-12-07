# Bumper

Automatic semantic versioning based on conventional commits, optimised for AI-generated commit messages.

## Installation

```bash
npm install -g @davegarvey/bumper
# or
npx @davegarvey/bumper
```

## Usage

```bash
# Run in your project root
bump

# Or with npx
npx bump

# Push to remote
bump --push

# Create git tag
bump --tag

# Raw mode (output only version, dry run)
bump --raw

# With explicit options overrides
bump --tag --tag-prefix "release-v"
bump --commit-prefix "chore(release): bump"
bump --preset git --tag

# Show help
bump --help
```

## Configuration

You can use a file-based configuration for Bumper instead of CLI arguments.

Create `.versionrc.json` in your project root:

```json
{
  "packageFiles": ["package.json", "client/package.json"],
  "commitPrefix": "chore: bump version",
  "tagPrefix": "v",
  "push": false,
  "tag": false,
  "preset": "node"
}
```

### Configuration Options

- **`packageFiles`**: Array of package.json files to update (default: `["package.json"]`)
- **`commitPrefix`**: Prefix for version bump commits (default: `"chore: bump version"`)
- **`tagPrefix`**: Prefix for git tags (default: `"v"`)
- **`push`**: Whether to push commits/tags to remote (default: `false`)
- **`tag`**: Whether to create git tags for versions (default: `false`)
- **`preset`**: Versioning strategy to use (default: `"node"`). Options:
  - `"node"`: Updates `package.json` and `package-lock.json`
  - `"git"`: Tracks version via git tags only (no file updates)

### Best Practices

- **Branch Protection**: Protect your main branch and require CI checks to pass
- **Conventional Commits**: Ensure all commits follow [conventional commit format](https://www.conventionalcommits.org/en/v1.0.0/)
- **Monorepos**: Use `packageFiles` array for multiple packages
- **CI Permissions**: Grant write access to contents/commits for automated releases

## GitHub Actions

In a typical CI/CD scenario, bumper runs automatically when PRs are merged to main:

```yaml
name: Release
on:
  pull_request:
    types: [closed]
    branches: [main]

jobs:
  release:
    if: github.event.pull_request.merged == true
    runs-on: ubuntu-latest
    permissions:
      contents: write      # Required for pushing commits/tags
      pull-requests: read  # Required for PR info
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0  # Required for commit analysis
    - uses: actions/setup-node@v4
      with:
        node-version: 18
        cache: npm
    - run: npm ci
    - run: npm test
    - run: npm run lint
    - name: Configure Git
      run: |
        git config user.name "github-actions[bot]"
        git config user.email "github-actions[bot]@users.noreply.github.com"
    - name: Install bumper
      run: npm install @davegarvey/bumper
    - name: Bump version and release
      run: npx bump --push --tag
```

### CI Best Practices

- **Permissions**: Add `contents: write` permission for automated commits/tags
- **Branch Protection**: Require CI checks and restrict direct pushes to main
- **Testing**: Always run tests before releasing
- **Fetch Depth**: Use `fetch-depth: 0` for complete commit history analysis

## How It Works

1. Analyzes commits since last tag
2. Determines version bump (major/minor/patch) based on conventional commits
3. Updates package.json files
4. Creates git commit
5. Optionally creates git tag
6. Optionally pushes to remote

## Commit Types

- `feat:` → minor bump
- `fix:`, `refactor:`, `perf:` → patch bump
- Any type with `!` or `BREAKING CHANGE` → major bump
- `docs:`, `test:`, `chore:`, `config:` → no bump

## Troubleshooting

### Common Issues

**"Author identity unknown"**

- **Solution**: Configure git identity in CI before running bumper
- **Example**: Add git config step as shown in CI workflow

**"auto-version: not found"**

- **Solution**: Install bumper locally before running: `npm install @davegarvey/bumper`
- **Why**: npx may not resolve bins from remote packages reliably

**No version bump on merge**

- **Check**: Ensure PR contains conventional commits with `feat:`, `fix:`, etc.
- **Check**: Verify CI has write permissions to repository
- **Check**: Confirm `fetch-depth: 0` in checkout action

**Invalid config file**

- **Solution**: Ensure `.versionrc.json` contains valid JSON
- **Note**: Empty or invalid files fall back to defaults with a warning

### Getting Help

- Check commit format with conventional commits specification
- Verify CI permissions and branch protection rules
- Test locally with `bump` for debugging (pushing is disabled by default)

## For AI Users

This tool is optimised for AI-generated commit messages that follow conventional commit format. See [.github/prompts/sc.prompt.md](.github/prompts/sc.prompt.md) for an example prompt that generates commits compatible with bumper.
