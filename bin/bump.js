#!/usr/bin/env node

import { getLastTag, getCommitsSinceTag, commitChanges, createTag, push } from '../lib/git.js';
import { analyseCommits } from '../lib/analyser.js';
import { getCurrentVersion, bumpVersion, updatePackageFiles } from '../lib/versioner.js';
import { loadConfig } from '../lib/config.js';

function buildActionMessage(baseAction, shouldTag, isPush) {
    const actions = [baseAction];
    if (shouldTag) actions.push(isPush ? 'and tags' : 'and tagged');
    return `${actions.join(' ')}${isPush ? '.' : ' locally.'}`;
}

if (process.argv.includes('--help') || process.argv.includes('-h')) {
    console.log(`Usage: bump [options]

Options:
  --push        Push changes to remote
  --tag         Create git tag for the version
  --help, -h    Show this help message

Automatic semantic versioning based on conventional commits.`);
    process.exit(0);
}

const config = loadConfig();
const shouldPush = process.argv.includes('--push') || config.push;
const shouldTag = process.argv.includes('--tag') || config.tag;

try {
    const currentVersion = getCurrentVersion(config.packageFiles[0]);
    console.log(`Current version: ${currentVersion}`);

    const lastTag = getLastTag();
    console.log(`Last tag: ${lastTag || 'none'}`);

    const commits = getCommitsSinceTag(lastTag);

    if (commits.length === 0) {
        console.log('No commits since last tag.');
        process.exit(0);
    }

    const { bump, triggeringCommits } = analyseCommits(commits);
    console.log(`Version bump: ${bump.toUpperCase()}`);

    if (bump === 'none') {
        console.log('No version bump required.');
        process.exit(0);
    }

    console.log('Triggering commits:');
    triggeringCommits.forEach(c => console.log(`  - ${c}`));

    const newVersion = bumpVersion(currentVersion, bump);
    const updatedFiles = updatePackageFiles(config.packageFiles, newVersion);

    console.log(`Updated to ${newVersion}`);

    commitChanges(newVersion, updatedFiles);
    if (shouldTag) {
        createTag(newVersion);
    }

    if (shouldPush) {
        push();
        console.log(buildActionMessage('Pushed changes', shouldTag, true));
    } else {
        console.log(buildActionMessage('Committed', shouldTag, false));
    }

} catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
}