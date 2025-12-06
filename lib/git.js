import { execSync } from 'child_process';

function runGitCommand(cmd, options = {}) {
    try {
        return execSync(cmd, {
            encoding: 'utf8',
            stdio: ['pipe', 'pipe', 'pipe'],
            ...options
        });
    } catch (error) {
        const stderr = error.stderr?.toString().trim() || '';
        const stdout = error.stdout?.toString().trim() || '';
        const message = stderr || stdout || error.message;
        throw new Error(`Git command failed: ${cmd}\n${message}`);
    }
}

export function getLastTag() {
    try {
        return runGitCommand('git describe --tags --abbrev=0').trim();
    } catch {
        return null;
    }
}

export function getCommitsSinceTag(lastTag) {
    const cmd = lastTag
        ? `git log ${lastTag}..HEAD --pretty=%s`
        : 'git log --pretty=%s';

    return runGitCommand(cmd)
        .trim()
        .split('\n')
        .filter(msg => msg.trim() !== '');
}

export function commitChanges(version, files) {
    runGitCommand(`git add ${files.join(' ')}`);
    runGitCommand(`git commit -m "chore: bump version to ${version}"`);
}

export function createTag(version) {
    runGitCommand(`git tag v${version}`);
}

export function push() {
    runGitCommand('git push && git push --tags');
}