# Start Here

Genotype is a programming language designed to help developers synchronize types between TypeScript, Rust, and Python (more languages coming soon).

## Learn the Genotype Language

To get familiar with the Genotype programming language, see [Quick Genotype Language Tour](language.md). It will lead you through all the language features, show you how to use them, and link to more detailed documentation.

To learn more about the specific Genotype targets, see:

- [TypeScript Overview](typescript.md)
- [Rust Overview](rust.md)
- [Python Overview](python.md)

## Quick Start

Follow the steps below to get started with Genotype or, alternatively, to play with Genotype without installing it, try [Genotype Playground](https://genotype-lang.org/playground).

### 1. Install the CLI

Install the Genotype CLI to get started:

**Linux/macOS**

```sh
curl -fsSL https://genotype-lang.org/install.sh | sh
```

**Windows**

```powershell
irm https://genotype-lang.org/install.ps1 | iex
```

See [Genotype Installation](https://genotype-lang.org/docs/getting-started/installation/) for more options.

### 2. Bootstrap a New Project

Run `gt init` to bootstrap a new Genotype project. It will ask you a few questions and generate `genotype.toml` and source files for your project:

```sh
gt init
```

See [the CLI reference](cli.md) for more information about the Genotype CLI.

### 3. Edit Type Files

Create a new file `.type` in the `src` directory and add your types, for example `src/user.type`:

```type
User: {
  name: FullName,
  email: string,
}

FullName: {
  first: string,
  last?: string,
}
```

See [the Quick Tour](language.md) for more information about the language.

### 4. Build

Run `gt build` to generate code for the targets configured in [`genotype.toml`](configuration.md):

```sh
gt build
```

The generated files go into the output directories selected during initialization. Run the command again after editing your types.

See [the CLI reference](cli.md#gt-build) for more build options.

## Working with AI Agents

Run `gt skill install` to install the Genotype agent skill. `gt init` also has the option to install the skill during project setup. See [Agent Skill](https://genotype-lang.org/docs/toolchain/skill/) for more info.

The documentation is also available as [llms.txt](https://genotype-lang.org/llms.txt), an index for AI tools, and [llms-full.txt](https://genotype-lang.org/llms-full.txt), the full documentation in a single text file. Give your agent these links when it needs documentation without browsing the website.
