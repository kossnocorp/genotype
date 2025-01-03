import { getGenotypeLspBinPath } from "genotype-lsp";
import * as vscode from "vscode";
import { workspace } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
  const binPath = getGenotypeLspBinPath(context.extensionPath);

  const serverOptions: ServerOptions = {
    command: binPath,
    args: [],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "genotype" }],
    outputChannelName: "Genotype LSP",
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/genotype.toml"),
    },
  };

  client = new LanguageClient(
    "genotype",
    "Genotype",
    serverOptions,
    clientOptions
  );

  client.start();
}

export async function deactivate(): Promise<void> {
  await client?.stop();
  client = undefined;
}
