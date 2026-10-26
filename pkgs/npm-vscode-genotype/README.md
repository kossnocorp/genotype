# Genotype for VS Code

Language support for [Genotype](https://genotype-lang.org), a language for defining types and generating code in multiple programming languages.

The extension adds syntax highlighting rules and a formatter for `.type` files.

## Formatting

To enable formatting on save, add this to your VS Code settings:

```json
{
  "[genotype]": {
    "editor.defaultFormatter": "nocorp.genotype",
    "editor.formatOnSave": true
  }
}
```

> **Warning:** Language Server Protocol (LSP) support is not implemented yet. The extension doesn't currently provide type diagnostics, code completion, or go-to-definition.

Install from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=nocorp.genotype) or [Open VSX](https://open-vsx.org/extension/nocorp/genotype).

See the [Genotype documentation](https://genotype-lang.org/docs) to get started.
