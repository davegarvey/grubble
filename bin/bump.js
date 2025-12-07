#!/usr/bin/env node

import minimist from 'minimist';
import { getLastTag, getCommitsSinceTag, commitChanges, createTag, push } from '../lib/git.js';
import { analyseCommits } from '../lib/analyser.js';
import { bumpVersion } from '../lib/versioner.js';
import { loadConfig } from '../lib/config.js';
import { loadStrategy } from '../lib/strategies/loader.js';

const argv = minimist(process.argv.slice(2), {
    boolean: ['push', 'tag', 'raw', 'help'],
    string: ['preset', 'tag-prefix', 'commit-prefix', 'package-files'],
    alias: {
        h: 'help',
        p: 'push',
        t: 'tag'
    }
});

if (argv.help) {
    console.log(`Usage: bump [options]

Options:
  --push, -p            Push changes to remote
  --tag, -t             Create git tag for the version
  --raw                 Output only the new version string (dry run, no changes)
  --preset <name>       Versioning strategy (node, git)
  --tag-prefix <str>    Prefix for git tags (default: v)
  --commit-prefix <str> Prefix for commit messages
  --package-files <...> Comma-separated list of files to update (node preset only)
  --help, -h            Show this help message

Automatic semantic versioning based on conventional commits.`);
    process.exit(0);
}

// Load config from file
let config = loadConfig();

// Override with CLI arguments
if (argv.preset) config.preset = argv.preset;
if (argv['tag-prefix']) config.tagPrefix = argv['tag-prefix'];
if (argv['commit-prefix']) config.commitPrefix = argv['commit-prefix'];
if (argv['package-files']) config.packageFiles = argv['package-files'].split(',');
if (argv.push) config.push = true;
if (argv.tag) config.tag = true;

const isRaw = argv.raw;

// Force settings for raw mode
if (isRaw) {
    config.raw = true;
    config.push = false;
    config.tag = false;
}

// Logger that respects raw mode
const log = (msg) => {
    if (!isRaw) console.log(msg);
};
const error = (msg) => {
    if (!isRaw) console.error(msg);
    else process.exit(1);
};

try {
    const strategy = loadStrategy(config);

    // In raw mode, we might not have a package file, but Strategy handles that.
    // For NodeStrategy, it expects one. 
    // If --raw is passed, loadStrategy forces GitStrategy, which doesn't need package files.

    const currentVersion = strategy.getCurrentVersion();
    log(`Current version: ${currentVersion}`);

    const lastTag = getLastTag();
    log(`Last tag: ${lastTag || 'none'}`);

    const commits = getCommitsSinceTag(lastTag);

    if (commits.length === 0) {
        log('No commits since last tag.');
        // If raw mode, we still might want to output the current version? 
        // Or if no commits, maybe no bump?
        // Usually if no changes, no new version.
        // User request: "output the version". Likely the *next* version.
        // If no bump, then next version == current version?
        // Let's assume correct behavior is "calculate next version".
        // If no commits, bump is 'none'.
    }

    const { bump, triggeringCommits, unknownCommits } = analyseCommits(commits, config);
    log(`Version bump: ${bump.toUpperCase()}`);

    if (bump === 'none') {
        log('No version bump required.');
        if (isRaw) {
            // If no bump required, maybe output current version? 
            // Or nothing? 
            // "determine version bumps" -> if no bump, it's the same version.
            console.log(currentVersion);
        }
        process.exit(0);
    }

    log('Triggering commits:');
    if (!isRaw) {
        triggeringCommits.forEach(c => log(`  - ${c}`));
    }

    // Warn about unknown commit types
    if (unknownCommits.length > 0 && !isRaw) {
        log('Warning: The following commits have unknown or unconfigured types and did not trigger a version bump:');
        unknownCommits.forEach(c => log(`  - ${c}`));
        log('Consider configuring these types in .versionrc.json or using standard Conventional Commits types.');
    }

    const newVersion = bumpVersion(currentVersion, bump);

    if (isRaw) {
        console.log(newVersion);
        process.exit(0);
    }

    const updatedFiles = strategy.updateFiles(newVersion);
    log(`Updated to ${newVersion}`);

    if (updatedFiles.length > 0) {
        commitChanges(newVersion, updatedFiles, config);
    }

    if (config.tag) {
        createTag(newVersion, config);
    }

    if (config.push) {
        push();
        const baseAction = 'Pushed changes';
        const actions = [baseAction];
        if (config.tag) actions.push('and tags');
        log(`${actions.join(' ')}.`);
    } else {
        // Only log if we effectively did something (commit or tag)
        if (updatedFiles.length > 0 || config.tag) {
            const baseAction = 'Committed';
            const actions = [baseAction];
            if (config.tag) actions.push('and tagged');
            log(`${actions.join(' ')} locally.`);
        }
    }

} catch (err) {
    // Determine if it's safe to print error (don't pollute stdout in raw mode?)
    // But stderr is fine.
    error('Error: ' + err.message);
    process.exit(1);
}