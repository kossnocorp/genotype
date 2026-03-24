import { getBinPath } from "genotype-lsp";
import * as vscode from "vscode";
import { workspace } from "vscode";
import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(_context: vscode.ExtensionContext) {
  const config = workspace.getConfiguration("genotype");
  const command = config.get<Partial<Executable>>("server.executable");

  const binPath = getBinPath();

  const serverOptions: Executable = {
    command: binPath,
    args: [],
    ...command,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "genotype" }],
    outputChannelName: "Genotype",
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

export function deactivate() {
  return client?.stop();
}
