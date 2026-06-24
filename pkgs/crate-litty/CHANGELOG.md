# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning].

This change log follows the format documented in [Keep a CHANGELOG].

[semantic versioning]: http://semver.org/
[keep a changelog]: http://keepachangelog.com/

## Unreleased

### Changed

- Deprecated `Literals`, `SerializeLiterals`, and `DeserializeLiterals` derives in favor of `serde_literals` with explicit Serde derives. It allows using Serde attributes like `rename` and `rename_all`.

- Deprecated unit struct attributes `literal`, `serialize_literal`, and `deserialize_literal` in favor of `serde_literal` with explicit Serde derives.

### Added

- Added `serde_literals` for structs and enums with explicit `Serialize` and/or `Deserialize` derives. The behavior is the same as the deprecated `Literals`, `SerializeLiterals`, and `DeserializeLiterals` derives.

- Added `serde_literal` for unit structs with explicit `Serialize` and/or `Deserialize` derives. The behavior is the same as the deprecated `literal`, `serialize_literal`, and `deserialize_literal` attributes.

- Added ability to rename field names e.g. `#[literals(request_method("POST", rename = "requestMethod"))]`.

## v0.4.0 - 2026-05-03

### Fixed

- Fixed lifetime issues in generated code.

### Changed

- Made macros generate unique identifiers for internal enum helpers to avoid possible name collisions.

## v0.3.0 - 2026-03-25

Initial public version.
