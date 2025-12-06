#!/usr/bin/env node

import { getLastTag, getCommitsSinceTag, commitChanges, createTag, push } from '../lib/git.js';
import { analyseCommits } from '../lib/analyser.js';
import { getCurrentVersion, bumpVersion, updatePackageFiles } from '../lib/versioner.js';
import { loadConfig } from '../lib/config.js';

const config = loadConfig();
const shouldPush = !process.argv.includes('--no-push') && config.push;

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
    createTag(newVersion);

    if (shouldPush) {
        push();
        console.log('Pushed changes and tags.');
    } else {
        console.log('Committed and tagged locally.');
    }

} catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
}