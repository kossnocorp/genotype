import { GtwmFormatters, initSync } from "@genotype-lang/formatters-wasm";
import * as vscode from "vscode";

let initialization: Promise<void> | undefined;

function initialize(): Promise<void> {
  initialization ??= Promise.resolve(
    vscode.workspace.fs.readFile(
      vscode.Uri.joinPath(vscode.Uri.file(__dirname), "genotype_formatters_wasm_bg.wasm"),
    ),
  )
    .then((bytes) => {
      initSync({ module: bytes });
    })
    .catch((error) => {
      initialization = undefined;
      throw error;
    });
  return initialization;
}

export class GenotypeFormattingProvider implements vscode.DocumentFormattingEditProvider {
  async provideDocumentFormattingEdits(
    document: vscode.TextDocument,
    _options: vscode.FormattingOptions,
    cancellation: vscode.CancellationToken,
  ): Promise<vscode.TextEdit[]> {
    if (cancellation.isCancellationRequested || document.isClosed) return [];

    const version = document.version;
    const source = document.getText();
    const stale = () =>
      cancellation.isCancellationRequested || document.isClosed || document.version !== version;

    try {
      await initialize();
      if (stale()) return [];

      const formatter = new GtwmFormatters();
      let formatted: string;
      try {
        formatted = formatter.formatGt(source);
      } finally {
        formatter.free();
      }

      if (stale() || formatted === source) return [];
      return [
        vscode.TextEdit.replace(
          new vscode.Range(document.positionAt(0), document.positionAt(source.length)),
          formatted,
        ),
      ];
    } catch (error) {
      if (!stale()) {
        void vscode.window.showErrorMessage(`Genotype formatting failed: ${String(error)}`);
      }
      return [];
    }
  }
}
