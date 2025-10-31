# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI/CD workflows
- Security audit and dependency scanning
- Automated documentation deployment
- Cross-platform release builds

## [1.0.0] - 2025-10-31

### Added
- Initial release of GenFuncHash
- Custom hash algorithm implementation (`h = 5527 * h + 7 * char; v = h & 0xFFFF; h ^= v * v`)
- Command-line interface with clap
- Single function hash generation (`-f` flag)
- Batch processing from input file (`-i` flag)
- C array output format with proper indexing
- TOML configuration file support (`-c` flag)
- Verbose mode (`-v` flag)
- Comprehensive error handling and validation
- Cross-platform support (Windows, macOS, Linux)

### Documentation
- Complete API documentation
- Architecture and design documentation
- User guide with examples
- AI coding agent instructions

### Testing
- Unit tests for hash algorithm
- Integration tests for CLI functionality
- Property-based testing for hash consistency
- Output format validation tests

## [0.1.0] - 2025-10-31

### Added
- Project initialization
- Basic project structure
- Initial Cargo.toml configuration

---

**Template for future releases:**

## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Now removed features

### Fixed
- Any bug fixes

### Security
- Vulnerability fixes