# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-11

### Added

- Initial release of the `cas_codes` crate
- `CAS` struct for representing Chemical Abstracts Service Registry Numbers
- Parsing from string format (`NNNNNNN-NN-N`) with comprehensive validation
- Support for both ASCII hyphens (`-`) and en-dashes (`–`) as separators
- Checksum validation according to official CAS algorithm
- Comprehensive error handling with `InvalidString` and `InvalidChecksum` variants
- Iterator over individual digits via `digits()` method
- Component access methods: `first()`, `second()`, `check_digit()`
- Display implementation for formatted output
- Utility functions in `utils` module including standalone `checksum()` function
- Optional Serde support for JSON serialization/deserialization
- Optional PGRX support for PostgreSQL extension development
- Optional Diesel ORM integration via `diesel_pgrx` feature
- Comprehensive documentation with examples
- Full test coverage including property-based testing with real CAS number dataset

### Features

- **Zero-cost abstractions**: CAS numbers stored as single `u32` for efficiency
- **Memory efficient**: Only 4 bytes per CAS number
- **Fast comparisons**: Direct integer comparison for equality/ordering
- **Robust validation**: Strict format and checksum validation
- **Detailed errors**: Clear error messages for debugging
- **Multiple input formats**: Support for different hyphen characters

### Performance

- Constant time access to components
- Minimal memory footprint
- Cache-friendly data structure

[Unreleased]: https://github.com/earth-metabolome-initiative/emi-monorepo/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/earth-metabolome-initiative/emi-monorepo/releases/tag/v0.1.0
