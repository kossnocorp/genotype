<p align="center">
  <a href="https://genotype-lang.org">
    <picture>
      <source srcset="./assets/exports/logotype-dark.svg" media="(prefers-color-scheme: dark)">
      <source srcset="./assets/exports/logotype-light.svg" media="(prefers-color-scheme: light)">
      <img src="./assets/exports/logotype-light.svg" alt="Genotype logotype" width="190px">
    </picture>
  </a>
</p>

<p align="center">
  <big>Programming Language</big>
</p>

<p align="center">
  A programming language that transpiles to idiomatic TypeScript/Rust/Python types, allowing effortless type-safe interop between languages.
</p>

<p align="center">
  <a href="https://genotype-lang.org/">Website</a> •
  <a href="https://genotype-lang.org/docs/getting-started/">Docs</a> •
  <a href="https://genotype-lang.org/docs/language/">Language Tour</a> •
  <a href="https://genotype-lang.org/playground/">Playground</a> •
  <a href="https://discord.gg/vXCfjVbj9J">Discord</a>
</p>

## Why Genotype?

Genotype is built for teams working with multiple programming languages (e.g., TypeScript front-end with Rust back-end), building API clients, file formats and standards, gradual language migration, etc.

Genotype is a small language that you can learn in 15 minutes, but it is feature-rich and allows expressing the most complex data structures. It has generics, supports recursive data types, and allows target-specific fine-tuning via annotations.

It isn't opinionated and adapts to the ecosystem instead of creating one. Want to generate Zod schemas instead of plan types? No problem! Need a ready-to-ship package or want to generate code right into existing modules? Can do! Need support for legacy Python versions? Absolutely.

Unlike most machine-generated code, the code Genotype produces is clean, follows the idiomatic naming for types and files, and follows the common conventions.

And of course, Genotype is AI agent-ready and provides [skill](https://genotype-lang.org/docs/toolchain/skill/), [llms.txt](https://genotype-lang.org/llms.txt), and leaves code comment breadcrumbs to help steer LLMs into using it correctly.

### Why Not Protobuf or Similar?

Genotype indeed overlaps with Protocol Buffers, but it doesn't impose the transport or storage format. It focuses on producing clean idiomatic types with minimal to no runtime instead of optimizing data transfer performance. TypeScript has no runtime at all. Rust and Python use standard Serde and Pydantic to close the gaps.

As a result, it better fits when developing API clients and web app client-server interop. It's more flexible and allows more complex data types and data shapes Protobuf cannot express directly.

## Installation

### macOS/Linux

Install the Genotype CLI on macOS or Linux by running the following command in your terminal:

```bash
curl -fsSL https://genotype-lang.org/install.sh | sh
```

### Windows

Install the Genotype CLI on Windows by running the following command in PowerShell:

```powershell
irm https://genotype-lang.org/install.ps1 | iex
```

---

See [Installation Reference](https://genotype-lang.org/docs/getting-started/installation/) for more options, such as Cargo Binstall, building from source, etc.

## Quick Start

Try it out:

```sh
mkdir hello-world && cd hello-world
gt init
gt build
```

This will create a build a new project with a language guide demonstrating the Genotype syntax and features.

---

See the [Getting Started](https://genotype-lang.org/docs/getting-started/) guide for more details.

## License

[MIT © Sasha Koss](https://koss.nocorp.me/mit/)
