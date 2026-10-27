# `genotype_lsp`

This is an internal crate of the Genotype Programming Language. It is not intended to be used by external users.

The `gt lsp` CLI command hosts this library's stdio language server. It provides
project-wide rename, definition navigation, semantic tokens, live diagnostics,
Markdown hover documentation, completion with auto-imports, and import quick fixes. There is no
separate LSP executable or platform-specific npm distribution.

See the [Genotype website](https://genotype-lang.org) for more information about the project.

The VS Code extension uses the `genotype/generatedSymbols` request to map source
type declarations to configured TypeScript, Rust, and Python output modules.
Parameters are `{ uri, position? }`: a source position selects a rename identity;
a generated module URI without a position returns its source declarations. The
extension locates native declarations and coordinates the installed rename providers.
