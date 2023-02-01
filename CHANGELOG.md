# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2023-01-31
### Changed
- Switch from `error-stack` to `snafu` for error handling.
- All errors now derive `Clone` trait..

## [0.2.0] - 2022-10-15
### Added
- `micromath` feature

### Changed
- Now `micromath` is used by default instead of `libm`.
- Changed license from `MIT` to `MIT OR Apache-2.0`

## [0.1.0] - 2022-10-15
- Initial release

[Unreleased]: https://github.com/chvllad/max31855-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/chvllad/max31855-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/chvllad/max31855-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/chvllad/max31855-rs/releases/tag/v0.1.0