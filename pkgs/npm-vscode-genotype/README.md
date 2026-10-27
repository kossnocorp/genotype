# Genotype for VS Code

Language support for [Genotype](https://genotype-lang.org), a language for defining types and generating code in multiple programming languages.

## Cross-language rename

Renaming a Genotype type with **F2** or **Rename Symbol** also requests rename edits
from the installed language providers for targets enabled in `genotype.toml`:
TypeScript, Rust (rust-analyzer), and Python (ty or Pylance). Generated declarations
and references known to those providers are edited together with the `.type`
source, without invoking `gt build`.

From TypeScript, Rust, or Python, right-click a generated type or a reference to it
and choose **Genotype: Rename Across Languages**. The command is also in the
command palette and the **Refactor…** menu. Native F2 in those languages remains
their own rename operation: VS Code has no supported cross-extension symbol-rename
callback ([upstream proposal](https://github.com/microsoft/vscode/issues/327750)).

Build the generated sources once before using this feature, and ensure each enabled
target's language server can analyze them and their consumers in the workspace.
The mapping follows saved configuration, including custom root/output paths,
package layouts, and source-file naming. It supports top-level Genotype type
declarations, including TypeScript's same-named Zod value/type pair. Fields,
generated-only helper declarations, and module/file renames are not mapped.
Genotype-local generic parameters and import aliases retain their source-only rename.
If a required provider or generated declaration is unavailable, the rename reports
the problem and does not apply partial edits.

The extension adds [syntax](#syntax-highlighting) and [semantic](#semantic-highlighting) highlighting, [a formatter](#formatter) for `.type` files and [a language server](#language-server) with rich IDE features.

Install from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=nocorp.genotype) or [Open VSX](https://open-vsx.org/extension/nocorp/genotype).

See the [Genotype documentation](https://genotype-lang.org/docs) to get started.

## Syntax Highlighting

Syntax highlighting is automatically provided for `.type` files regardless of `gt` CLI or `genotype.toml` presence.

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

### Semantic Highlighting

To enable semantic highlighting provided by the [Language Server](#language-server), use the global VS Code setting:

```json
{
  "editor.semanticHighlighting.enabled": true
}
```

## License

[MIT © Sasha Koss](https://koss.nocorp.me/mit/)
