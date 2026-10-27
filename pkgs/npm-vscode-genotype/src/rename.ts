import * as vscode from "vscode";
import type { LanguageClient, Middleware } from "vscode-languageclient/node";

interface Position {
  line: number;
  character: number;
}
interface Location {
  uri: string;
  range: { start: Position; end: Position };
}
interface GeneratedSymbol {
  name: string;
  source: Location;
  targets: { uri: string; language: string }[];
}

const command = "genotype.renameAcrossLanguages";
const selector = ["genotype", "typescript", "typescriptreact", "rust", "python"];

function position(value: Position) {
  return new vscode.Position(value.line, value.character);
}

/** Collect edits before applying any of them, so providers see the same names. */
export function mergeRenameEdits(edits: readonly vscode.WorkspaceEdit[]): vscode.WorkspaceEdit {
  const merged = new vscode.WorkspaceEdit();
  for (const edit of edits) {
    for (const [uri, changes] of edit.entries()) {
      const existing = merged.get(uri);
      for (const change of changes) {
        const duplicate = existing.find((other) => other.range.isEqual(change.range));
        if (duplicate?.newText === change.newText) continue;
        if (
          duplicate ||
          existing.some((other) => {
            const intersection = other.range.intersection(change.range);
            return (
              intersection && (!intersection.isEmpty || other.range.isEmpty || change.range.isEmpty)
            );
          })
        ) {
          throw new Error(`Language servers returned conflicting rename edits in ${uri.fsPath}.`);
        }
        existing.push(change);
      }
      merged.set(uri, existing);
    }
  }
  return merged;
}

export class CrossLanguageRename {
  constructor(private readonly client: () => LanguageClient) {}

  readonly middleware: Middleware = {
    provideRenameEdits: async (document, at, newName, token, next) => {
      return this.stable(async () => {
        const mappings = await this.mappings(document.uri, at);
        const source = await next(document, at, newName, token);
        if (!source || token.isCancellationRequested) return undefined;
        const edits = [source];
        for (const mapping of mappings) {
          edits.push(...(await this.targetEdits(mapping, newName, token)));
        }
        if (token.isCancellationRequested) return undefined;
        return mergeRenameEdits(edits);
      });
    },
  };

  register(context: vscode.ExtensionContext) {
    context.subscriptions.push(
      vscode.commands.registerCommand(command, async (uri?: vscode.Uri, at?: vscode.Position) => {
        try {
          const editor = vscode.window.activeTextEditor;
          uri ??= editor?.document.uri;
          at ??= editor?.selection.active;
          if (!uri || !at) return;
          const document = await vscode.workspace.openTextDocument(uri);
          const mapping = await this.findSource(document, at);
          if (!mapping)
            throw new Error(
              "This symbol is not a mapped Genotype type declaration. Use Rename Symbol for local symbols.",
            );
          const newName = await vscode.window.showInputBox({
            title: "Genotype: Rename Across Languages",
            prompt: `Rename ${mapping.name} in Genotype and its configured language targets`,
            value: mapping.name,
          });
          if (!newName || newName === mapping.name) return;
          // Re-resolve after the input box: the source may have changed while it was open.
          const current = await this.findSource(document, at);
          if (!current || JSON.stringify(current) !== JSON.stringify(mapping)) {
            throw new Error("The symbol changed while entering its name. Retry the rename.");
          }
          const sourceUri = vscode.Uri.parse(mapping.source.uri);
          await vscode.workspace.openTextDocument(sourceUri);
          // This invokes our middleware, which adds all configured target edits.
          const edit = await vscode.commands.executeCommand<vscode.WorkspaceEdit>(
            "vscode.executeDocumentRenameProvider",
            sourceUri,
            position(mapping.source.range.start),
            newName,
          );
          if (!edit?.size) throw new Error("The Genotype server did not return rename edits.");
          if (!(await vscode.workspace.applyEdit(edit)))
            throw new Error("VS Code could not apply the rename.");
        } catch (error) {
          void vscode.window.showErrorMessage(
            `Genotype rename: ${error instanceof Error ? error.message : error}`,
          );
        }
      }),
    );
    context.subscriptions.push(
      vscode.languages.registerCodeActionsProvider(
        selector.map((language) => ({ scheme: "file", language })),
        {
          provideCodeActions: async (document, range, context, token) => {
            if (context.only && !context.only.contains(vscode.CodeActionKind.RefactorRewrite))
              return [];
            const mapping = await this.findSource(document, range.start).catch(() => undefined);
            if (!mapping || token.isCancellationRequested) return [];
            const action = new vscode.CodeAction(
              "Rename across Genotype languages…",
              vscode.CodeActionKind.RefactorRewrite,
            );
            action.command = {
              command,
              title: action.title,
              arguments: [document.uri, range.start],
            };
            return [action];
          },
        },
        { providedCodeActionKinds: [vscode.CodeActionKind.RefactorRewrite] },
      ),
    );
  }

  private async mappings(uri: vscode.Uri, at?: vscode.Position): Promise<GeneratedSymbol[]> {
    const client = this.client();
    await client.start();
    return client.sendRequest("genotype/generatedSymbols", { uri: uri.toString(), position: at });
  }

  private async findSource(
    document: vscode.TextDocument,
    at: vscode.Position,
  ): Promise<GeneratedSymbol | undefined> {
    if (document.languageId === "genotype") return (await this.mappings(document.uri, at))[0];
    const pending = [{ uri: document.uri, at }];
    const visited = new Set<string>();
    const found = new Map<string, GeneratedSymbol>();
    // Follow re-export/import aliases using semantic definitions, never a workspace
    // name search (unrelated modules frequently export identically named types).
    while (pending.length && visited.size < 32) {
      const current = pending.shift()!;
      const key = `${current.uri}:${current.at.line}:${current.at.character}`;
      if (visited.has(key)) continue;
      visited.add(key);
      const doc = await vscode.workspace.openTextDocument(current.uri);
      const mappings = await this.mappings(current.uri);
      let matched = false;
      for (const mapping of mappings) {
        const declarations = await this.declarations(doc, mapping.name);
        if (declarations.some((range) => range.contains(current.at))) {
          found.set(JSON.stringify(mapping.source), mapping);
          matched = true;
        }
      }
      if (matched) continue;
      const definitions = await vscode.commands.executeCommand<
        (vscode.Location | vscode.LocationLink)[]
      >("vscode.executeDefinitionProvider", current.uri, current.at);
      for (const definition of definitions ?? []) {
        pending.push(
          "targetUri" in definition
            ? {
                uri: definition.targetUri,
                at: (definition.targetSelectionRange ?? definition.targetRange).start,
              }
            : { uri: definition.uri, at: definition.range.start },
        );
      }
    }
    if (found.size > 1)
      throw new Error("The generated declaration maps to multiple Genotype sources.");
    return found.values().next().value;
  }

  private async declarations(document: vscode.TextDocument, name: string): Promise<vscode.Range[]> {
    const symbols = await vscode.commands.executeCommand<
      (vscode.DocumentSymbol | vscode.SymbolInformation)[]
    >("vscode.executeDocumentSymbolProvider", document.uri);
    const ranges: vscode.Range[] = [];
    for (const symbol of symbols ?? []) {
      if (symbol.name !== name) continue;
      // VS Code returns hybrid objects with both DocumentSymbol and
      // SymbolInformation fields. Prefer hierarchy over container metadata.
      if (!("selectionRange" in symbol) && symbol.containerName) continue;
      const range = "selectionRange" in symbol ? symbol.selectionRange : symbol.location.range;
      // Top-level declarations only. Child properties and generated helper types
      // are not source aliases even when they happen to share the same name.
      if (document.getText(range) === name) {
        ranges.push(range);
        continue;
      }
      const declaration = "range" in symbol ? symbol.range : symbol.location.range;
      const resolved = await this.declarationNames(document, declaration, name);
      if (resolved.length === 1) ranges.push(resolved[0]);
    }
    return ranges;
  }

  private async declarationNames(
    document: vscode.TextDocument,
    declaration: vscode.Range,
    name: string,
  ) {
    const ranges: vscode.Range[] = [];
    const text = document.getText(declaration);
    const start = document.offsetAt(declaration.start);
    // A flat symbol's range can include keywords, documentation, and references.
    // Locate candidates only inside that named declaration, then ask the native
    // definition provider for the actual identifier. Never rename a text match.
    for (const match of text.matchAll(/[A-Za-z_$][\w$]*/g)) {
      if (match[0] !== name) continue;
      const at = document.positionAt(start + match.index);
      const definitions = await vscode.commands.executeCommand<
        (vscode.Location | vscode.LocationLink)[]
      >("vscode.executeDefinitionProvider", document.uri, at);
      for (const definition of definitions ?? []) {
        const uri = "targetUri" in definition ? definition.targetUri : definition.uri;
        const selected =
          "targetUri" in definition
            ? (definition.targetSelectionRange ?? definition.targetRange)
            : definition.range;
        if (
          uri.toString() !== document.uri.toString() ||
          !declaration.contains(selected.start) ||
          !declaration.contains(selected.end)
        )
          continue;
        let identifier = selected;
        if (document.getText(identifier) !== name) {
          // Some definition providers also return whole declarations. Prepare
          // rename supplies the token range without applying any edit.
          const prepared = await vscode.commands
            .executeCommand<{ range: vscode.Range; placeholder: string }>(
              "vscode.prepareRename",
              document.uri,
              at,
            )
            .then(undefined, () => undefined);
          if (
            !prepared ||
            !selected.contains(prepared.range.start) ||
            !selected.contains(prepared.range.end)
          )
            continue;
          identifier = prepared.range;
        }
        if (
          document.getText(identifier) === name &&
          !ranges.some((other) => other.isEqual(identifier))
        )
          ranges.push(identifier);
      }
    }
    return ranges;
  }

  private async targetEdits(
    mapping: GeneratedSymbol,
    newName: string,
    token: vscode.CancellationToken,
  ) {
    const edits: vscode.WorkspaceEdit[] = [];
    for (const target of mapping.targets) {
      if (token.isCancellationRequested) return [];
      const uri = vscode.Uri.parse(target.uri);
      const owners = (await this.mappings(uri)).filter(
        (candidate) => candidate.name === mapping.name,
      );
      if (
        owners.length !== 1 ||
        JSON.stringify(owners[0].source) !== JSON.stringify(mapping.source)
      ) {
        throw new Error(
          `The generated declaration ${mapping.name} in ${uri.fsPath} has an ambiguous source mapping.`,
        );
      }
      const document = await vscode.workspace.openTextDocument(uri).then(undefined, () => {
        throw new Error(
          `Generated ${target.language} source is missing: ${uri.fsPath}. Build the project before cross-language rename.`,
        );
      });
      const ranges = await this.declarations(document, mapping.name);
      if (!ranges.length) {
        const symbols = await vscode.commands.executeCommand<
          (vscode.DocumentSymbol | vscode.SymbolInformation)[]
        >("vscode.executeDocumentSymbolProvider", uri);
        const matching = (symbols ?? []).filter((symbol) => symbol.name === mapping.name);
        const detail = matching.length
          ? `${matching.length} matching outline symbol(s), but their declaration ranges could not be resolved.`
          : `The outline provider returned ${symbols?.length ?? 0} top-level symbols, none named ${mapping.name}.`;
        throw new Error(`Cannot locate ${mapping.name} in ${uri.fsPath}. ${detail}`);
      }
      // Zod can emit both a value and a type with the same name. Ask the native
      // provider about each declaration and deduplicate its shared edits.
      for (const range of ranges) {
        const edit = await vscode.commands.executeCommand<vscode.WorkspaceEdit>(
          "vscode.executeDocumentRenameProvider",
          uri,
          range.start,
          newName,
        );
        if (!edit?.get(uri).some((change) => change.range.contains(range.start))) {
          throw new Error(
            `The ${target.language} language server could not rename ${mapping.name} in ${uri.fsPath}.`,
          );
        }
        edits.push(edit);
      }
    }
    return edits;
  }

  private async stable<T>(run: () => Promise<T>): Promise<T> {
    let changed = false;
    const listener = vscode.workspace.onDidChangeTextDocument((event) => {
      if (event.contentChanges.length) changed = true;
    });
    try {
      const result = await run();
      if (changed)
        throw new Error("Documents changed while computing rename edits. Retry the rename.");
      return result;
    } finally {
      listener.dispose();
    }
  }
}
