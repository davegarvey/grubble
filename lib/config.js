import fs from 'fs';
import path from 'path';

const DEFAULT_CONFIG = {
    packageFiles: ['package.json'],
    commitPrefix: 'chore: bump version',
    tagPrefix: 'v',
    push: true
};

export function loadConfig(cwd = process.cwd()) {
    const configPath = path.join(cwd, '.versionrc.json');

    if (fs.existsSync(configPath)) {
        try {
            const content = fs.readFileSync(configPath, 'utf8').trim();
            if (content) {
                const userConfig = JSON.parse(content);
                return { ...DEFAULT_CONFIG, ...userConfig };
            }
        } catch (error) {
            // If config file exists but is invalid, warn and use defaults
            console.warn(`Warning: Invalid .versionrc.json file, using default config: ${error.message}`);
        }
    }

    return DEFAULT_CONFIG;
}