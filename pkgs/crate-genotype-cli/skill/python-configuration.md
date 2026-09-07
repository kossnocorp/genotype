# Python Target Configuration

To configure Python target, use `[py]` in `genotype.toml`.

This configuration reference lists all available Python configuration options.

See the [Genotype Configuration](configuration.md) for global settings and [Common Target Options](configuration.md#common-target-options).

## Basic

### `py.enabled` - Enable Target

Set to `true` to generate Python. Defaults to `false`; see [enable target](configuration.md#targetenabled---enable-target).

```toml
[py]
enabled = true
```

## Generation

### `py.version` - Python Version

`version` selects the Python syntax generation mode:

- `"latest"` (default): Python 3.13 and later, using modern syntax such as `type` aliases.
- `"legacy"`: Python 3.8 and later, using older typing constructs and [typing-extensions](https://pypi.org/project/typing-extensions/) where needed.

```toml
[py]
enabled = true
version = "latest"
```

**Note:**

`py.version` selects the language mode. To set the Python package's release version, use the
[global version](configuration.md#version---package-version) or [Python
manifest](#pymanifest---pyprojecttoml).

## Package

### `py.package` - Package Generation

Overrides package generation for Python. Inherits the global setting when omitted; see [target package generation](configuration.md#targetpackage---package-generation).

### `py.dist` - Output Directory

Defaults to `"py"`, relative to the global output directory. See [target output directory](configuration.md#targetdist---output-directory).

### `py.manager` - Package Manager

`manager` selects the generated [Python manifest](#pymanifest---pyprojecttoml) format:

- `"poetry"` (default): Poetry metadata under `[tool.poetry]`.
- `"uv"`: Package metadata under `[project]`.

```toml
[py]
enabled = true
version = "latest"
manager = "uv"
```

### `py.module` - Module Name

`module` sets the generated Python package's importable module name. It defaults to `"module"`:

```toml
[py]
enabled = true
version = "latest"
module = "bookstore_types"
```

With [package generation](configuration.md#targetpackage---package-generation) enabled, source files go into this directory inside the [target output directory](configuration.md#targetdist---output-directory), e.g., `dist/py/bookstore_types`.

### `[py.manifest]` - pyproject.toml

[Manifest options](configuration.md#targetmanifest---package-metadata) in `[py.manifest]` follow the `pyproject.toml` structure. The default package name uses `kebab-case`. The location of package name and version overrides depends on [manager](#pymanager---package-manager):

**Poetry**

```toml
[py]
enabled = true
version = "latest"
manager = "poetry"

[py.manifest.tool.poetry]
name = "bookstore-types"
version = "0.2.0"
description = "Shared bookstore types"

[py.manifest.tool.poetry.dependencies]
bookstore-shared = "^1.0.0"
```

**uv**

```toml
[py]
enabled = true
version = "latest"
manager = "uv"

[py.manifest.project]
name = "bookstore-types"
version = "0.2.0"
description = "Shared bookstore types"
```

With `uv`, Genotype replaces `project.dependencies` with its detected runtime dependencies. Additional dependencies configured in that array aren't preserved.

## Modules

### `[py.dependencies]` - External Modules

Values in [dependencies](configuration.md#targetdependencies---external-modules) are Python module import paths, which may differ from their distribution names:

```toml
[py]
enabled = true
version = "latest"

[py.dependencies]
shared_types = "bookstore_shared"
```

## Formatting

### `py.formatters` - Formatters

Adds formatters for Python; defaults to `[]`. See [target formatters](configuration.md#targetformatters---formatters) for execution order and [formatter configuration](configuration.md#formatters---formatters) for commands and presets.
