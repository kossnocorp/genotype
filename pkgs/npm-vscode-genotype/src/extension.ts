import * as vscode from "vscode";
import { GenotypeFormattingProvider } from "./formatter";

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider(
      { language: "genotype" },
      new GenotypeFormattingProvider(),
    ),
  );
}

export function deactivate() {}
