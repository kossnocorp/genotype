# Genotype for VS Code

Language support for [Genotype](https://genotype-lang.org), a language for defining types and generating code in multiple programming languages.

The extension adds [syntax](#syntax-highlighting) and [semantic](#semantic-highlighting) highlighting, [a formatter](#formatter) for `.type` files and [a language server](#language-server) with rich IDE features.

Install from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=nocorp.genotype) or [Open VSX](https://open-vsx.org/extension/nocorp/genotype).

See the [Genotype documentation](https://genotype-lang.org/docs) to get started.

## Syntax Highlighting

Syntax highlighting is automatically provided for `.type` files regardless of the presence of the `gt` CLI or `genotype.toml` project configuration.

## Formatter

To enable formatting on save, add this to your VS Code settings:

```json
{
  "[genotype]": {
    "editor.defaultFormatter": "nocorp.genotype",
    "editor.formatOnSave": true
  }
}
```

## Language Server

[Install the Genotype CLI](https://genotype-lang.org/docs/getting-started/installation/) and make sure `gt` is available on VS Code's PATH.

The extension starts `gt lsp` automatically when you open a Genotype file.

To use a custom CLI location, set `genotype.server.executable`:

```json
{
  "genotype.server.executable": {
    "command": "/path/to/gt",
    "args": ["lsp"]
  }
}
```

### Symbols Rename

The extension allows two-way symbol renaming between Genotype `.type` files and the generated code in supported target languages.

When renaming from `.type` files, use `F2` or the **Rename Symbol** command to trigger the cross-language rename.

When renaming a Genotype-generated symbol from a target language file, use the **Genotype: Rename Across Languages** command or **Rename Across Languages** context menu item to trigger the cross-language rename back to the corresponding `.type` file.

### Semantic Highlighting

To enable semantic highlighting provided by the [Language Server](#language-server), use the global VS Code setting:

```json
{
  "editor.semanticHighlighting.enabled": true
}
```

## License

[MIT © Sasha Koss](https://koss.nocorp.me/mit/)
