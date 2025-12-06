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
  "packageFiles": [
    "package.json",
    "client/package.json"
  ],
  "push": true
}
```

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
    - name: Install bumper
      run: npm install @davegarvey/bumper
    - name: Bump version and release
      run: npx bump
```

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

## For AI Users

This tool is optimised for AI-generated commit messages that follow conventional commit format. See [.github/prompts/sc.prompt.md](.github/prompts/sc.prompt.md) for an example prompt that generates commits compatible with bumper.
