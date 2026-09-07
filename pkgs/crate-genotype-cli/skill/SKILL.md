---
name: genotype
description: Define shared types in Genotype .type files and generate TypeScript, Rust, or Python code. Use when creating or editing Genotype schemas, configuring genotype.toml, or running the gt CLI.
license: MIT
---

# Genotype

Genotype is a language that allows you to define types and data structures and generate code implementing them in multiple programming languages, such as TypeScript, Rust, Python, and more.

The workflow is:

1. Edit types in `.type` files.
2. Generate code for the enabled targets using Genotype CLI: `gt build`.

To create a new Genotype project or configure an existing one, edit `genotype.toml` in the project root.

A monorepo may contain multiple Genotype projects, each with its own `genotype.toml` and source files.

Never edit generated code directly. Always modify the `.type` files and regenerate.

## Language

Genotype source files `.type` look like this:

```type
User: {
  name: UserName,
  email?: string,
  roles: [string],
}

UserName: {
  first: string,
  last?: string
}
```

Read the [Language Tour](language.md) for the overview of the Genotype language and its features.

## Targets

Genotype converts the shared type definitions in `.type` files into code for the enabled target languages. Each target has its own conventions and options, which can be configured in `genotype.toml`.

See target overviews for more information on how Genotype translates shared type definitions into code for each language:

- [TypeScript Overview](typescript.md)
- [Rust Overview](rust.md)
- [Python Overview](python.md)

## Configuration

Genotype projects are configured using the `genotype.toml` file that looks like this:

```toml
name = "my_project"

[ts]
enabled = true

[rs]
enabled = true

[py]
enabled = true
```

Read [Configuration Reference](configuration.md) for details on all available configuration options.

See target configuration references for target-specific options:

- [TypeScript Configuration](typescript-configuration.md)
- [Rust Configuration](rust-configuration.md)
- [Python Configuration](python-configuration.md)

## Additional Information

- Read [CLI Reference](cli.md) for more information beyond basic `gt build` usage.
- Read [Getting Started](getting-started.md) for initial setup.
