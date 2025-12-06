import { describe, it, expect } from 'vitest';
import { analyseCommits } from '../lib/analyser.js';

describe('analyseCommits', () => {
    it('should detect major bump for breaking changes', () => {
        const commits = ['feat!: breaking change'];
        const result = analyseCommits(commits);
        expect(result.bump).toBe('major');
    });

    it('should detect minor bump for features', () => {
        const commits = ['feat: add new feature'];
        const result = analyseCommits(commits);
        expect(result.bump).toBe('minor');
    });

    it('should detect patch bump for fixes', () => {
        const commits = ['fix: correct bug'];
        const result = analyseCommits(commits);
        expect(result.bump).toBe('patch');
    });

    it('should ignore chore commits', () => {
        const commits = ['chore: update deps'];
        const result = analyseCommits(commits);
        expect(result.bump).toBe('none');
    });

    it('should filter out its own version bump commits', () => {
        const commits = [
            'feat: add new feature',
            'chore: bump version to 1.1.0',
            'fix: correct bug'
        ];
        const result = analyseCommits(commits);
        expect(result.bump).toBe('minor');
        expect(result.triggeringCommits).toHaveLength(2);
        expect(result.triggeringCommits).toContain('Minor: feat: add new feature');
        expect(result.triggeringCommits).toContain('Patch: fix: correct bug');
    });
});