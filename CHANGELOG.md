# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- [**BREAKING**] `rust_version` key in cargo package. The Minimum Supported Rust Version has
  been a part of the documentation until this point, but this addition could
  break builds of `varj`.

### Changed

#### [**BREAKING**] `VarjMap::insert` parameter types

`VarjMap::insert` now takes keys and values that implement `Into<Cow<'a, str>>`
instead of `Into<String>`. This means it doesn't allocate a `String` if the key
and/or value outlives the `VarjMap`.

##### Migration

For the majority of cases this wont cause a break: any caller that passes in an
`&str` or `String` directly will continue to work. However, if the argument's
type is limited to the same constraint as the old API (`Into<String>`), then the
caller must change it's invocation of `VarjMap::insert`.

## [1.1.0] 2022-10-28

### Changed

- Rename `VarjMap` method `parse` to `render`. Keep a deprecated `parse` method.

### Documentation

- Add MSRV policy.

## [1.0.3] 2022-10-20

### Documentation

- Fix rustdoc links.
- Modernize readme.
- Add minimum supported rust version 1.56.1

## [1.0.2] 2022-01-07

### Documentation

- Fix broken links.

## [1.0.1] 2022-01-07

### Added

- `VarjMap` conversion from `HashMap::<String, String>`.

## [1.0.0] 2021-12-29

### Changed

- [**BREAKING**] `VarjMap` from type alias to wrapper type.
- [**BREAKING**] `parse` function now a method on `VarjMap`.
- [**BREAKING**] Rust edition from 2018 to 2021.

### Documentation

- Add examples.

## [0.1.0] 2020-03-24

### Added

- `VarjMap` type alias for `HashMap::<String, String>`.
- `parse` function to interpolate `VarjMap` values into a mustache-like
  template.

[Unreleased]: https://github.com/sonro/varj/compare/v1.0.3...HEAD
[1.0.3]: https://github.com/sonro/varj/releases/tag/v1.0.3
[1.0.2]: https://github.com/sonro/varj/releases/tag/v1.0.2
[1.0.1]: https://github.com/sonro/varj/releases/tag/v1.0.1
[1.0.0]: https://github.com/sonro/varj/releases/tag/v1.0.0
[0.1.0]: https://github.com/sonro/varj/releases/tag/v0.1.0
