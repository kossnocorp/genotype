# Genotype language support

🚧 Work in progress

## Development

To run the LSP server in the debug mode, add this to your VS Code settings:

```json
{
  "genotype.server.executable": {
    "command": "cargo watch --quiet -x \"run --quiet\"",
    "options": {
      "cwd": "/ABSOLUTE/PATH/TO/genotype/lsp",
      "shell": true
    }
  }
}
```
