# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2019-05-29

Removed old encipher/decipher code and replaced it with `encipher_file` and `decipher_file`.  This brings this into line with the CLI tool.

## [0.1.1] - 2019-05-28

Rehomed with non-code updates to helper scripts.

Changed output name to `hitwasm_xtea`.

## [0.1.1] - 2019-04-30

Bug fixes and simplifies.

### Fixed

  * CRC32 interface (simplified it and made it behave as expected in all cases).

### Removed

  * CRC32 finalisation call.

## [0.1.0] - 2019-04-29

Initial public release.

### Added

  * cargo skeletal code (cargo etc.);
  * Horrible WASM code;
  * Horrible CRC32 (software) implementation that works in WASM;
  * Horrible, horrible, comments.