import { describe, it, expect } from 'vitest';
import { getCurrentVersion, bumpVersion, updatePackageFiles } from '../lib/versioner.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

describe('versioner', () => {
    describe('getCurrentVersion', () => {
        it('should read version from package.json', () => {
            const testPkgPath = path.join(__dirname, 'fixtures', 'package.json');
            const version = getCurrentVersion(testPkgPath);
            expect(version).toBe('1.2.3');
        });
    });

    describe('bumpVersion', () => {
        it('should bump major version', () => {
            expect(bumpVersion('1.2.3', 'major')).toBe('2.0.0');
        });

        it('should bump minor version', () => {
            expect(bumpVersion('1.2.3', 'minor')).toBe('1.3.0');
        });

        it('should bump patch version', () => {
            expect(bumpVersion('1.2.3', 'patch')).toBe('1.2.4');
        });

        it('should return same version for none', () => {
            expect(bumpVersion('1.2.3', 'none')).toBe('1.2.3');
        });
    });

    describe('updatePackageFiles', () => {
        it('should update version in package files', () => {
            const testPkgPath = path.join(__dirname, 'fixtures', 'test-package.json');
            const originalContent = JSON.stringify({ version: '1.0.0', name: 'test' }, null, 2) + '\n';
            fs.writeFileSync(testPkgPath, originalContent);

            const updated = updatePackageFiles([testPkgPath], '2.0.0');
            expect(updated).toEqual([testPkgPath]);

            const content = JSON.parse(fs.readFileSync(testPkgPath, 'utf8'));
            expect(content.version).toBe('2.0.0');

            // Cleanup
            fs.unlinkSync(testPkgPath);
        });

        it('should skip non-existent files', () => {
            const updated = updatePackageFiles(['non-existent.json'], '2.0.0');
            expect(updated).toEqual([]);
        });
    });
});