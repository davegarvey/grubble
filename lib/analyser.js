import { DEFAULT_CONFIG } from './config.js';

const BUMP_COMMIT_PREFIX = 'chore: bump version';

export function analyseCommits(commits, config = DEFAULT_CONFIG) {
    const substantiveCommits = commits.filter(
        msg => !msg.startsWith(BUMP_COMMIT_PREFIX)
    );

    if (substantiveCommits.length === 0) {
        return { bump: 'none', triggeringCommits: [] };
    }

    let bump = 'none';
    const triggeringCommits = [];
    const unknownCommits = [];
    // Allow any lowercase word as type
    const commitTypeRegex = /^([a-z]+)(?:\([^)]+\))?(!?):/;
    const typeMapping = config.types || DEFAULT_CONFIG.types;

    for (const msg of substantiveCommits) {
        const match = msg.match(commitTypeRegex);
        if (!match) continue;

        const [, type, hasExclamation] = match;
        const hasBreaking = hasExclamation === '!' ||
            msg.toLowerCase().includes('breaking change');

        let commitBump = 'none';

        // Breaking changes are always major unless configured otherwise? 
        // Standard conventional commits says ! or BREAKING CHANGE footer is major.
        if (hasBreaking) {
            commitBump = 'major';
        } else if (typeMapping[type]) {
            commitBump = typeMapping[type];
        } else {
            // Unknown type, add to unknown commits for warning
            unknownCommits.push(msg);
        }

        // Update overall bump (major > minor > patch)
        if (commitBump === 'major' ||
            (commitBump === 'minor' && bump !== 'major') ||
            (commitBump === 'patch' && bump === 'none')) {
            bump = commitBump;
        }

        if (commitBump !== 'none') {
            const label = commitBump.charAt(0).toUpperCase() + commitBump.slice(1);
            triggeringCommits.push(`${label}: ${msg}`);
        }
    }

    return { bump, triggeringCommits, unknownCommits };
}