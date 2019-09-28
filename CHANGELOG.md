# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.2.0] - 2019-09-28

### Added
- Method to read calibrated measurement of UVA, UVB and UV index.
- Methods to set the integration time and dynamic setting.
- Method to change the operating mode and triggering a measurement when on active force mode.

### Changed
- [breaking-change] Renamed VEML6075 -> Veml6075 to comply with Rust naming conventions.
- [breaking-change] Renamed methods to read raw measurements

## [0.1.0] - 2018-10-13

This is the initial release to crates.io. All changes will be documented in this CHANGELOG.

[Unreleased]: https://github.com/eldruin/veml6075-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/veml6075-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eldruin/veml6075-rs/releases/tag/v0.1.0
