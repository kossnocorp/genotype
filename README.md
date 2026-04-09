# Genotype Programming Language

A programming language that transpiles to idiomatic TypeScript/Rust/Python types, allowing effortless type-safe interop between languages.

Its goal is to help developers working on projects that use multiple languages or require API clients in multiple languages.

🚧 Work in progress, **follow for updates on [Twitter](https://twitter.com/kossnocorp).

## Installation

Currently, Genotype can only be installed via Cargo:

```sh
cargo install genotype_cli
```

## Quick Start

Try it out:

```sh
mkdir hello-world && cd hello-world
gt init
gt build
```

This will create a build a new project with a language guide demonstrating the Genotype syntax and features.

## License

[MIT © Sasha Koss](https://koss.nocorp.me/mit/)