## ADDED Requirements

### Requirement: Action hard-fails when release checksum is missing
The GitHub Action SHALL exit with a non-zero status (via a `::error::` annotation and `exit 1`) when a `.sha256` file for a release asset cannot be downloaded.

#### Scenario: Missing checksum file
- **WHEN** the action attempts to download `grubble-linux-x86_64.tar.gz.sha256` for a release
- **AND** the download fails (HTTP 404 or similar)
- **THEN** the action writes a `::error::` annotation to the log
- **AND** the action exits with a non-zero status

### Requirement: Action hard-fails on checksum mismatch
The GitHub Action SHALL exit with a non-zero status (via a `::error::` annotation and `exit 1`) when the computed checksum of a downloaded asset does not match the value in the `.sha256` file.

#### Scenario: Tampered or corrupted download
- **WHEN** the action downloads a release asset
- **AND** the SHA-256 of the downloaded file does not match the value in the corresponding `.sha256` file
- **THEN** the action writes a `::error::` annotation to the log
- **AND** the action exits with a non-zero status

### Requirement: Action downloads checksum from the same release
The GitHub Action SHALL derive the checksum URL from the same release version used for the binary download, ensuring checksums are always matched to the correct asset.

#### Scenario: Checksum URL derivation
- **WHEN** the action downloads `grubble-linux-x86_64.tar.gz` from release `v5.0.0`
- **THEN** the action downloads `grubble-linux-x86_64.tar.gz.sha256` from release `v5.0.0`
- **AND** uses that `.sha256` file to verify the binary

### Requirement: Action verifies checksums on all supported platforms
The GitHub Action SHALL verify the SHA-256 checksum on Linux, macOS, and Windows builds using the platform's available verification tool.

#### Scenario: Unix verification
- **WHEN** the action runs on a Linux or macOS runner
- **THEN** the action uses `shasum -a 256 -c` to verify the downloaded asset

#### Scenario: Windows verification
- **WHEN** the action runs on a Windows runner
- **THEN** the action uses `certutil -hashfile ... SHA256` to verify the downloaded asset
