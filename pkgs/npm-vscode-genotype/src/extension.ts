import * as vscode from "vscode";
import { GenotypeFormattingProvider } from "./formatter";
import { Executable, LanguageClient } from "vscode-languageclient/node";
import { CrossLanguageRename } from "./rename";

let client: LanguageClient | undefined;

export function activateFormatting(context: Pick<vscode.ExtensionContext, "subscriptions">) {
  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider(
      { language: "genotype" },
      new GenotypeFormattingProvider(),
    ),
  );
}

export function activate(context: vscode.ExtensionContext) {
  activateFormatting(context);
  const rename = new CrossLanguageRename(() => client!);
  const watcher = vscode.workspace.createFileSystemWatcher("**/*.{type,toml}");
  context.subscriptions.push(watcher);
  const executable = vscode.workspace
    .getConfiguration("genotype")
    .get<Partial<Executable>>("server.executable");
  client = new LanguageClient(
    "genotype",
    "Genotype",
    {
      command: "gt",
      args: ["lsp"],
      ...executable,
    },
    {
      documentSelector: [{ scheme: "file", language: "genotype" }],
      synchronize: { fileEvents: watcher },
      middleware: rename.middleware,
    },
  );
  context.subscriptions.push(client);
  rename.register(context);
  void client.start().catch((error: unknown) => {
    void vscode.window.showErrorMessage(
      `Could not start Genotype language server. Make sure gt is installed and on PATH. ${error}`,
    );
  });
}

export function deactivate() {
  return client?.stop();
}
