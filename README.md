# Genotype

A programming language that lets you describe your data structure types once and compile them to other languages like TypeScript, Python or Rust.

Its goal is to help developers work on projects that use multiple languages, or require API clients in multiple languages.

## Installation

Currently, Genotype can be only installed from source code using Cargo:

```sh
cargo install genotype_cli
```

## Usage

Try it out:

```sh
mkdir hello-world && cd hello-world
gt init
gt build
```

This will create a build a new project with a language guide demonstrating the Genotype syntax and features.

### Watch mode

Right now, Genotype has no built-in watch mode, but you can any file watcher to run it on file changes. Give that it is super fast, it will fit most of the use cases.

Here's an example using [`entr`](http://eradman.com/entrproject/):

```sh
while sleep 0.1; do ls src/*.type | entr -d just build; done
```

## Roadmap

At this point, Genotype is actively using to develop a startup I co-founded. The roadmap is driven largely by the needs of this project.

The current goal is to polish the DX, so that it can be found on the internet, used by other people without my guidance in a real project. This includes:

- [ ] Website
- [ ] Documentation
- [ ] Improved VS Code extension
- [ ] Consistent error reporting

### Future

Some of the future goals in no particular order:

- [ ] Package manager that works in a monorepo
- [ ] Language server
- [ ] (Neo)Vim extension
- [ ] JetBrains extension
- [ ] Go language support

## Design

### Principles

The design principles are presented in the order of importance.

#### Unambiguity

Both the syntax and the described structures should be unambiguous.

While the syntax should limit ambiguity as much as possible, complete unambiguity should be achieved by the type system.

#### Portability

Data structures should be portable and representable in JSON.

#### Transparency

Genotype data structures should visually map to the data representation. It should be obvious what the JSON would look like.

#### Compatibility

It should be possible to express any structure in any target language. This means that not all target language structures can be necessarily expressed in Genotype, but any Genotype structure can be expressed in any target language.

#### Idiomatic

The resulting target language code should be idiomatic and consider the context where the target language is used.

#### Expressiveness

Genotype aims to be as expressive as possible while maintaining idiomaticity and compatibility with the target languages.

#### Practicality

Genotype should not be dogmatic and lean into practicality when necessary.

#### Longevity

The target language code should stay relevant for as long as possible. This means limiting the use of dependencies.
