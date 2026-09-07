# Rust Target Configuration

To configure Rust target, use `[rs]` in `genotype.toml`.

This configuration reference lists all available Rust configuration options.

See the [Genotype Configuration](configuration.md) for global settings and [Common Target Options](configuration.md#common-target-options).

## Basic

### `rs.enabled` - Enable Target

Set to `true` to generate Rust. Defaults to `false`; see [enable target](configuration.md#targetenabled---enable-target).

```toml
[rs]
enabled = true
```

## Generation

### `rs.derive` - Derive Traits

`derive` sets the base list of traits derived for generated structs and enums. It defaults to `["Debug", "Clone", "PartialEq"]`:

```toml
[rs]
enabled = true
derive = ["Debug", "Clone", "PartialEq", "Eq", "Hash"]
```

The configured list replaces the default list. Genotype also adds traits required by generated types, including [Serde](https://crates.io/crates/serde) serialization and deserialization derives. `Default` is currently omitted from union enum derives.

Including `Eq`, `Hash`, or `Ord` makes Genotype use ordered float wrappers for floating-point values so they can support those traits.

## Package

### `rs.package` - Package Generation

Overrides package generation for Rust. Inherits the global setting when omitted; see [target package generation](configuration.md#targetpackage---package-generation).

### `rs.dist` - Output Directory

Defaults to `"rs"`, relative to the global output directory. See [target output directory](configuration.md#targetdist---output-directory).

### `[rs.manifest]` - Cargo.toml

[Manifest options](configuration.md#targetmanifest---package-metadata) in `[rs.manifest]` follow the `Cargo.toml` structure. Set package metadata under `[rs.manifest.package]`:

```toml
[rs.manifest.package]
name = "bookstore_types"
version = "0.2.0"
edition = "2024"
license = "MIT"

[rs.manifest.dependencies]
shared_types = "1"
```

The generated crate has a `src/lib.rs` entry point and defaults to edition `"2024"`. Its default name uses `snake_case`. See [package generation](configuration.md#targetpackage---package-generation) for layout controls.

**Tip:**

Set `rs.manifest.package.edition` explicitly to lock the edition. Genotype warns when it is
omitted and Rust package generation is enabled.

## Modules

### `[rs.dependencies]` - External Modules

Values in [dependencies](configuration.md#targetdependencies---external-modules) are Rust crate paths used in generated imports:

```toml
[rs.dependencies]
shared_types = "bookstore_shared"
```

## Formatting

### `rs.formatters` - Formatters

Adds formatters for Rust; defaults to `[]`. See [target formatters](configuration.md#targetformatters---formatters) for execution order and [formatter configuration](configuration.md#formatters---formatters) for commands and presets.
