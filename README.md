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

# Don't push to remote
bump --no-push
# or
npx bump --no-push
```

## Configuration

Create `.versionrc.json` in your project root:

```json
{
  "packageFiles": ["package.json", "client/package.json"],
  "commitPrefix": "chore: bump version",
  "tagPrefix": "v",
  "push": true
}
```

### Configuration Options

- **`packageFiles`**: Array of package.json files to update (default: `["package.json"]`)
- **`commitPrefix`**: Prefix for version bump commits (default: `"chore: bump version"`)
- **`tagPrefix`**: Prefix for git tags (default: `"v"`)
- **`push`**: Whether to push commits/tags to remote (default: `true`)

### Best Practices

- **Branch Protection**: Protect your main branch and require CI checks to pass
- **Conventional Commits**: Ensure all commits follow [conventional commit format](https://conventionalcommits.org/)
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
      actions: read        # Required for workflow info
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
      run: npx bump
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
4. Creates git commit and tag
5. Optionally pushes to remote

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
- Test locally with `bump --no-push` for debugging

## For AI Users

This tool is optimised for AI-generated commit messages that follow conventional commit format. See [.github/prompts/sc.prompt.md](.github/prompts/sc.prompt.md) for an example prompt that generates commits compatible with bumper.
