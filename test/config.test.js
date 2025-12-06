import { describe, it, expect } from 'vitest';
import { loadConfig } from '../lib/config.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

describe('config', () => {
    describe('loadConfig', () => {
        it('should return default config when no config file exists', () => {
            const config = loadConfig('/non-existent');
            expect(config).toEqual({
                packageFiles: ['package.json'],
                commitPrefix: 'chore: bump version',
                tagPrefix: 'v',
                push: true
            });
        });

        it('should merge user config with defaults', () => {
            const testConfigPath = path.join(__dirname, 'fixtures', '.versionrc.json');
            const testConfig = {
                packageFiles: ['package.json', 'package-lock.json'],
                push: false
            };
            fs.writeFileSync(testConfigPath, JSON.stringify(testConfig, null, 2));

            const config = loadConfig(path.join(__dirname, 'fixtures'));
            expect(config).toEqual({
                packageFiles: ['package.json', 'package-lock.json'],
                commitPrefix: 'chore: bump version',
                tagPrefix: 'v',
                push: false
            });

            // Cleanup
            fs.unlinkSync(testConfigPath);
        });
    });
});