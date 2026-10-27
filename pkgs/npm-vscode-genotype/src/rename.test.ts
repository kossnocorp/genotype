import { beforeEach, expect, it, vi } from "vitest";
import type { LanguageClient } from "vscode-languageclient/node";

const state = vi.hoisted(() => ({
  execute: vi.fn(),
  send: vi.fn(),
  input: vi.fn(),
  apply: vi.fn(),
  error: vi.fn(),
  information: vi.fn(),
  showDocument: vi.fn(),
  implementations: vi.fn((_selector: unknown, _provider: any) => ({ dispose() {} })),
  commands: new Map<string, (...args: any[]) => any>(),
  documents: new Map<string, any>(),
  listeners: new Set<(event: any) => void>(),
}));

vi.mock("vscode", () => {
  class Position {
    constructor(
      public line: number,
      public character: number,
    ) {}
  }
  const compare = (a: Position, b: Position) => a.line - b.line || a.character - b.character;
  class Range {
    constructor(
      public start: Position,
      public end: Position,
    ) {}
    get isEmpty() {
      return compare(this.start, this.end) === 0;
    }
    isEqual(other: Range) {
      return compare(this.start, other.start) === 0 && compare(this.end, other.end) === 0;
    }
    contains(at: Position) {
      return compare(this.start, at) <= 0 && compare(at, this.end) <= 0;
    }
    intersection(other: Range) {
      const start = compare(this.start, other.start) >= 0 ? this.start : other.start;
      const end = compare(this.end, other.end) <= 0 ? this.end : other.end;
      return compare(start, end) <= 0 ? new Range(start, end) : undefined;
    }
  }
  const Uri = {
    parse: (value: string) => ({ toString: () => value, fsPath: value.replace("file://", "") }),
  };
  class WorkspaceEdit {
    private changes = new Map<string, any[]>();
    get size() {
      return this.changes.size;
    }
    get(uri: any) {
      return this.changes.get(uri.toString()) ?? [];
    }
    set(uri: any, edits: any[]) {
      this.changes.set(uri.toString(), edits);
    }
    entries() {
      return [...this.changes].map(([uri, edits]) => [Uri.parse(uri), edits]);
    }
  }
  return {
    Position,
    Range,
    Uri,
    WorkspaceEdit,
    commands: {
      executeCommand: state.execute,
      registerCommand: (id: string, fn: any) => {
        state.commands.set(id, fn);
        return { dispose() {} };
      },
    },
    workspace: {
      openTextDocument: async (uri: any) => {
        const doc = state.documents.get(uri.toString());
        if (!doc) throw new Error("Missing file");
        return doc;
      },
      applyEdit: state.apply,
      onDidChangeTextDocument: (fn: any) => {
        state.listeners.add(fn);
        return { dispose: () => state.listeners.delete(fn) };
      },
    },
    languages: {
      registerCodeActionsProvider: () => ({ dispose() {} }),
      registerImplementationProvider: state.implementations,
    },
    CodeActionKind: { RefactorRewrite: "refactor.rewrite" },
    window: {
      showInputBox: state.input,
      showErrorMessage: state.error,
      showInformationMessage: state.information,
      showTextDocument: state.showDocument,
    },
  };
});

import * as vscode from "vscode";
import { CrossLanguageRename, mergeRenameEdits } from "./rename";

const source = "file:///project/src/user.type";
const targets = [
  { uri: "file:///project/dist/ts/src/user.ts", language: "typescript" },
  { uri: "file:///project/dist/rs/src/user.rs", language: "rust" },
  { uri: "file:///project/dist/py/module/user.py", language: "python" },
];
const range = (start: number, end = start + 4) =>
  new vscode.Range(new vscode.Position(0, start), new vscode.Position(0, end));
const mapping = { name: "User", source: { uri: source, range: range(0) }, targets };
const token = { isCancellationRequested: false } as vscode.CancellationToken;
let rename: CrossLanguageRename;

function document(uri: string, languageId: string, text: string) {
  const doc = {
    uri: vscode.Uri.parse(uri),
    languageId,
    version: 1,
    offsetAt: (at: vscode.Position) => at.character,
    positionAt: (offset: number) => new vscode.Position(0, offset),
    getText: (selection?: vscode.Range) =>
      selection ? text.slice(selection.start.character, selection.end.character) : text,
  } as vscode.TextDocument;
  state.documents.set(uri, doc);
  return doc;
}

function edit(uri: string, start: number, newText = "Account") {
  const result = new vscode.WorkspaceEdit();
  result.set(vscode.Uri.parse(uri), [{ range: range(start), newText }]);
  return result;
}

async function forward(cancellation = token) {
  return rename.middleware.provideRenameEdits!(
    state.documents.get(source),
    new vscode.Position(0, 0),
    "Account",
    cancellation,
    async () => edit(source, 0),
  );
}

beforeEach(() => {
  vi.clearAllMocks();
  state.commands.clear();
  state.documents.clear();
  state.listeners.clear();
  document(source, "genotype", "User: string");
  document(targets[0].uri, "typescript", "export type User = string;");
  document(targets[1].uri, "rust", "pub type User = String;");
  document(targets[2].uri, "python", "type User = str");
  state.send.mockImplementation(async (_method, params) =>
    params.position || targets.some((t) => t.uri === params.uri) ? [mapping] : [],
  );
  state.execute.mockImplementation(async (command, uri, at, newName) => {
    const doc = state.documents.get(uri.toString());
    if (command === "vscode.executeDocumentSymbolProvider") {
      const start = doc.getText().indexOf("User");
      return [{ name: "User", selectionRange: range(start), children: [] }];
    }
    if (command === "vscode.executeDefinitionProvider") return [];
    if (command === "vscode.executeImplementationProvider") {
      const provider = state.implementations.mock.calls.at(-1)![1] as vscode.ImplementationProvider;
      return provider.provideImplementation(doc, at, token);
    }
    if (command === "editor.action.peekLocations") return;
    if (command === "vscode.executeDocumentRenameProvider") {
      if (uri.toString() === source) return forward();
      return edit(uri.toString(), at.character, newName);
    }
    throw new Error(`Unexpected command: ${command}`);
  });
  state.input.mockResolvedValue("Account");
  state.apply.mockResolvedValue(true);
  rename = new CrossLanguageRename(
    () => ({ start: async () => {}, sendRequest: state.send }) as unknown as LanguageClient,
  );
});

async function implementations(cancellation = token) {
  rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
  const provider = state.implementations.mock.calls.at(-1)![1] as vscode.ImplementationProvider;
  return provider.provideImplementation(
    state.documents.get(source),
    new vscode.Position(0, 0),
    cancellation,
  ) as Promise<vscode.LocationLink[]>;
}

it("finds generated implementations in all configured languages without renaming", async () => {
  const locations = await implementations();
  expect(locations.map((location) => location.targetUri.toString())).toEqual(
    targets.map((target) => target.uri),
  );
  for (const location of locations) {
    expect(
      state.documents.get(location.targetUri.toString()).getText(location.targetSelectionRange),
    ).toBe("User");
  }
  expect(state.send).toHaveBeenCalledWith("genotype/generatedSymbols", {
    uri: source,
    position: new vscode.Position(0, 0),
  });
  expect(state.apply).not.toHaveBeenCalled();
  expect(
    state.execute.mock.calls.every(
      ([command]) => command === "vscode.executeDocumentSymbolProvider",
    ),
  ).toBe(true);
});

it("keeps available implementations when another output or provider is missing", async () => {
  state.documents.delete(targets[0].uri);
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, uri, ...args) => {
    if (uri.toString() === targets[1].uri) throw new Error("Provider unavailable");
    return execute(command, uri, ...args);
  });
  expect((await implementations()).map((location) => location.targetUri.toString())).toEqual([
    targets[2].uri,
  ]);
});

it("returns no implementations for unmapped symbols or cancelled requests", async () => {
  expect(
    await implementations({ isCancellationRequested: true } as vscode.CancellationToken),
  ).toEqual([]);
  expect(state.send).not.toHaveBeenCalled();
  state.send.mockResolvedValue([]);
  expect(await implementations()).toEqual([]);
  expect(state.execute).toHaveBeenCalledWith(
    "vscode.executeDefinitionProvider",
    expect.objectContaining({ fsPath: "/project/src/user.type" }),
    new vscode.Position(0, 0),
  );
});

it("follows local import aliases to the generated implementations of their definition", async () => {
  const original = "file:///project/src/original.type";
  state.send.mockImplementation(async (_method, params) =>
    params.uri === original ? [mapping] : [],
  );
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, ...args) => {
    if (command === "vscode.executeDefinitionProvider")
      return [{ uri: vscode.Uri.parse(original), range: range(0) }];
    return execute(command, ...args);
  });
  expect((await implementations()).length).toBe(3);
});

it("deduplicates implementation locations while retaining distinct same-named declarations", async () => {
  document(targets[0].uri, "typescript", "type User = string; const User = schema;");
  state.send.mockResolvedValue([{ ...mapping, targets: [targets[0], targets[0]] }]);
  state.execute.mockResolvedValue([
    { name: "User", selectionRange: range(5) },
    { name: "User", selectionRange: range(26) },
  ]);
  const locations = await implementations();
  expect(locations.map((location) => location.targetSelectionRange!.start.character)).toEqual([
    5, 26,
  ]);
});

it("discards implementations cancelled during provider lookup", async () => {
  const cancellation = { isCancellationRequested: false } as vscode.CancellationToken;
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (...args) => {
    (cancellation as { isCancellationRequested: boolean }).isCancellationRequested = true;
    return execute(...args);
  });
  expect(await implementations(cancellation)).toEqual([]);
});

it.each(targets)("opens Genotype source from $language declarations", async (target) => {
  rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
  const doc = state.documents.get(target.uri);
  await state.commands.get("genotype.goToSourceDefinition")!(
    doc.uri,
    new vscode.Position(0, doc.getText().indexOf("User")),
  );
  expect(state.showDocument).toHaveBeenCalledWith(
    expect.objectContaining({ fsPath: "/project/src/user.type" }),
    { selection: range(0) },
  );
  expect(state.apply).not.toHaveBeenCalled();
});

it.each(targets)(
  "shows every implementation including $language when invoked there",
  async (target) => {
    rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
    const doc = state.documents.get(target.uri);
    const at = new vscode.Position(0, doc.getText().indexOf("User"));
    await state.commands.get("genotype.findImplementationsAcrossLanguages")!(doc.uri, at);
    const peek = state.execute.mock.calls.find(
      ([command]) => command === "editor.action.peekLocations",
    )!;
    expect(peek[1]).toBe(doc.uri);
    expect(peek[2]).toBe(at);
    expect(peek[3].map((location: vscode.Location) => location.uri.toString())).toEqual(
      targets.map((target) => target.uri),
    );
    expect(state.apply).not.toHaveBeenCalled();
  },
);

it("follows a target-language reference back to its Genotype source", async () => {
  const consumer = document("file:///project/app.ts", "typescript", "let user: User;");
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, uri, ...args) => {
    if (
      command === "vscode.executeDefinitionProvider" &&
      uri.toString() === consumer.uri.toString()
    ) {
      return [{ uri: vscode.Uri.parse(targets[0].uri), range: range(12) }];
    }
    return execute(command, uri, ...args);
  });
  rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
  await state.commands.get("genotype.goToSourceDefinition")!(
    consumer.uri,
    new vscode.Position(0, 10),
  );
  expect(state.showDocument).toHaveBeenCalledOnce();
  expect(state.error).not.toHaveBeenCalled();
});

it("reports unmapped target symbols without opening unrelated source", async () => {
  state.send.mockResolvedValue([]);
  rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
  await state.commands.get("genotype.goToSourceDefinition")!(
    vscode.Uri.parse(targets[0].uri),
    new vscode.Position(0, 12),
  );
  expect(state.information).toHaveBeenCalledWith(expect.stringContaining("No Genotype source"));
  expect(state.showDocument).not.toHaveBeenCalled();
});

it("reports when mapped targets have no available implementations", async () => {
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, ...args) =>
    command === "vscode.executeImplementationProvider" ? [] : execute(command, ...args),
  );
  rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
  await state.commands.get("genotype.findImplementationsAcrossLanguages")!(
    vscode.Uri.parse(targets[0].uri),
    new vscode.Position(0, 12),
  );
  expect(state.information).toHaveBeenCalledWith(
    expect.stringContaining("No generated implementations"),
  );
  expect(
    state.execute.mock.calls.some(([command]) => command === "editor.action.peekLocations"),
  ).toBe(false);
});

it("collects Genotype, TypeScript, Rust and Python rename edits before applying anything", async () => {
  const edits = await forward();
  expect(edits?.size).toBe(4);
  for (const [uri, changes] of edits!.entries()) {
    let text = state.documents.get(uri.toString()).getText();
    for (const change of changes)
      text =
        text.slice(0, change.range.start.character) +
        change.newText +
        text.slice(change.range.end.character);
    expect(text).toContain("Account");
    expect(text).not.toContain("User");
  }
  expect(state.apply).not.toHaveBeenCalled();
  expect(state.listeners.size).toBe(0);
});

it.each(["flat", "whole selection", "hybrid", "whole definition"])(
  "locates branded type aliases from %s outline symbols",
  async (shape) => {
    const uri = vscode.Uri.parse(targets[0].uri);
    const text = "export type DefinitionId = string & { [definitionIdBrand]: true };";
    const name = "DefinitionId";
    const identifier = range(12, 24);
    const declaration = range(0, text.length);
    document(uri.toString(), "typescript", text);
    document(source, "genotype", "DefinitionId: string");
    state.send.mockResolvedValue([
      { name, source: { uri: source, range: range(0, 12) }, targets: [targets[0]] },
    ]);
    state.execute.mockImplementation(async (command, requested, at) => {
      if (command === "vscode.executeDocumentSymbolProvider")
        return [
          {
            name,
            location: { uri, range: declaration },
            ...(shape === "flat"
              ? {}
              : {
                  range: declaration,
                  selectionRange: shape === "hybrid" ? identifier : declaration,
                  containerName: "docs",
                  children: [],
                }),
          },
        ];
      if (command === "vscode.executeDefinitionProvider")
        return shape === "whole definition"
          ? [{ uri, range: declaration }]
          : [{ targetUri: uri, targetRange: declaration, targetSelectionRange: identifier }];
      if (command === "vscode.prepareRename") return { range: identifier, placeholder: name };
      if (command === "vscode.executeDocumentRenameProvider") {
        expect(at).toEqual(identifier.start);
        const result = new vscode.WorkspaceEdit();
        result.set(uri, [{ range: identifier, newText: "Account" }]);
        return result;
      }
      throw new Error(`Unexpected command ${command}`);
    });
    const result = await forward();
    expect(result?.get(uri)).toEqual([{ range: identifier, newText: "Account" }]);
    expect(state.apply).not.toHaveBeenCalled();
  },
);

it("does not rename lexical matches when semantic definition lookup fails", async () => {
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, uri, ...args) => {
    if (command === "vscode.executeDocumentSymbolProvider")
      return [
        {
          name: "User",
          location: { uri, range: range(0, state.documents.get(uri.toString()).getText().length) },
        },
      ];
    return execute(command, uri, ...args);
  });
  await expect(forward()).rejects.toThrow("matching outline symbol(s)");
  expect(
    state.execute.mock.calls.some(
      ([command]) => command === "vscode.executeDocumentRenameProvider",
    ),
  ).toBe(false);
});

it.each(targets)(
  "resolves a $language consumer to its generated declaration and applies a combined edit",
  async (target) => {
    const usage = document(`file:///project/app.${target.language}`, target.language, "User");
    const execute = state.execute.getMockImplementation()!;
    state.execute.mockImplementation(async (command, uri, ...args) => {
      if (
        command === "vscode.executeDefinitionProvider" &&
        uri.toString() === usage.uri.toString()
      ) {
        const start = state.documents.get(target.uri).getText().indexOf("User");
        return [
          {
            targetUri: vscode.Uri.parse(target.uri),
            targetSelectionRange: range(start),
            targetRange: range(start),
          },
        ];
      }
      return execute(command, uri, ...args);
    });
    rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
    await state.commands.get("genotype.renameAcrossLanguages")!(
      usage.uri,
      new vscode.Position(0, 0),
    );
    expect(state.error).not.toHaveBeenCalled();
    expect(state.apply).toHaveBeenCalledOnce();
    expect(state.apply.mock.calls[0][0].size).toBe(4);
  },
);

it("does not map an unrelated same-named symbol by name alone", async () => {
  const usage = document("file:///project/unrelated.ts", "typescript", "User");
  rename.register({ subscriptions: [] } as unknown as vscode.ExtensionContext);
  await state.commands.get("genotype.renameAcrossLanguages")!(usage.uri, new vscode.Position(0, 0));
  expect(state.input).not.toHaveBeenCalled();
  expect(state.apply).not.toHaveBeenCalled();
  expect(state.error).toHaveBeenCalledWith(expect.stringContaining("not a mapped Genotype"));
});

it("retains source-only rename for local aliases and generic parameters", async () => {
  state.send.mockResolvedValue([]);
  expect((await forward())?.size).toBe(1);
  expect(state.execute).not.toHaveBeenCalled();
});

it("aborts the entire rename if a configured target is missing or its provider refuses", async () => {
  state.documents.delete(targets[2].uri);
  await expect(forward()).rejects.toThrow("source is missing");
  expect(state.apply).not.toHaveBeenCalled();
  document(targets[2].uri, "python", "type User = str");
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, uri, ...args) =>
    command === "vscode.executeDocumentRenameProvider" && uri.toString() === targets[2].uri
      ? undefined
      : execute(command, uri, ...args),
  );
  await expect(forward()).rejects.toThrow("python language server could not rename");
});

it("discards cancelled requests and documents changed during native rename", async () => {
  expect(
    await forward({ isCancellationRequested: true } as vscode.CancellationToken),
  ).toBeUndefined();
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (...args) => {
    for (const listener of state.listeners) listener({ contentChanges: [{ text: "changed" }] });
    return execute(...args);
  });
  await expect(forward()).rejects.toThrow("Documents changed");
  expect(state.listeners.size).toBe(0);
});

it("deduplicates shared Zod edits and rejects conflicting edits", () => {
  expect(
    mergeRenameEdits([edit(source, 0), edit(source, 0)]).get(vscode.Uri.parse(source)),
  ).toHaveLength(1);
  expect(() => mergeRenameEdits([edit(source, 0), edit(source, 0, "Other")])).toThrow(
    "conflicting",
  );
  expect(() => mergeRenameEdits([edit(source, 0), edit(source, 2)])).toThrow("conflicting");
});

it("retains consumer reference edits returned by native providers", async () => {
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, uri, ...args) => {
    const result = await execute(command, uri, ...args);
    if (command === "vscode.executeDocumentRenameProvider") {
      const consumer = vscode.Uri.parse(uri.toString() + ".consumer");
      result.set(consumer, [{ range: range(10), newText: "Account" }]);
    }
    return result;
  });
  const result = await forward();
  expect(result?.size).toBe(7);
  for (const target of targets)
    expect(result?.get(vscode.Uri.parse(target.uri + ".consumer"))[0].newText).toBe("Account");
});

it("requests both Zod declarations and merges their shared edits", async () => {
  const uri = targets[0].uri;
  document(
    uri,
    "typescript",
    "export const User = z.string(); export type User = z.infer<typeof User>;",
  );
  const text = state.documents.get(uri).getText();
  const starts = [...text.matchAll(/User/g)].map((match) => match.index!);
  const execute = state.execute.getMockImplementation()!;
  state.execute.mockImplementation(async (command, target, ...args) => {
    if (target.toString() === uri) {
      if (command === "vscode.executeDocumentSymbolProvider")
        return starts.slice(0, 2).map((start) => ({ name: "User", selectionRange: range(start) }));
      if (command === "vscode.executeDocumentRenameProvider") {
        const result = new vscode.WorkspaceEdit();
        result.set(
          target,
          starts.map((start) => ({ range: range(start), newText: "Account" })),
        );
        return result;
      }
    }
    return execute(command, target, ...args);
  });
  expect((await forward())?.get(vscode.Uri.parse(uri))).toHaveLength(3);
  expect(
    state.execute.mock.calls.filter(
      ([command, target]) =>
        command === "vscode.executeDocumentRenameProvider" && target.toString() === uri,
    ),
  ).toHaveLength(2);
});
