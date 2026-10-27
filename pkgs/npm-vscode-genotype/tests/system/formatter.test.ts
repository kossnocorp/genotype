import { readFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { assert, describe, expect, it, vi } from "vitest";
import vm from "node:vm";
import { createRequire, isBuiltin } from "node:module";
import type * as vscode from "vscode";

describe("formatter", () => {
  it("registers a lazy formatter for file and unsaved Genotype documents", () => {
    const extension = activate();
    expect(extension.selector.language).toBe("genotype");
    expect(extension.selector.scheme).toBeUndefined();
    expect(extension.subscriptions).toHaveLength(1);
    expect(extension.read).not.toHaveBeenCalled();
  });

  it("formats the whole document using the bundled Wasm", async () => {
    const { provider, showErrorMessage } = activate();
    const edits = await provider.provideDocumentFormattingEdits(
      document("Example:{value:string}"),
      {},
      { isCancellationRequested: false },
    );
    expect(edits).toHaveLength(1);
    expect(edits[0].newText).toBe("Example: { value: string, }\n");
    expect(edits[0].range.start).toEqual({ line: 0, character: 0 });
    expect(edits[0].range.end).toEqual({ line: 0, character: 22 });
    expect(showErrorMessage).not.toHaveBeenCalled();
  });

  it("reuses initialization and skips unchanged documents", async () => {
    const { provider, read } = activate();
    const first = await provider.provideDocumentFormattingEdits(
      document("Value:string"),
      {},
      { isCancellationRequested: false },
    );
    const second = await provider.provideDocumentFormattingEdits(
      document(first[0].newText),
      {},
      { isCancellationRequested: false },
    );
    expect(second).toHaveLength(0);
    expect(read).toHaveBeenCalledTimes(1);
  });

  it("reports syntax errors without returning edits", async () => {
    const { provider, showErrorMessage } = activate();
    const edits = await provider.provideDocumentFormattingEdits(
      document("Value: {"),
      {},
      { isCancellationRequested: false },
    );
    expect(edits).toHaveLength(0);
    assert.match(showErrorMessage.mock.calls[0][0], /Genotype formatting failed:.*Syntax/);
  });

  it("does not initialize for an already cancelled request", async () => {
    const { provider, read } = activate();
    const edits = await provider.provideDocumentFormattingEdits(
      document("Value:string"),
      {},
      { isCancellationRequested: true },
    );
    expect(edits).toHaveLength(0);
    expect(read).not.toHaveBeenCalled();
  });

  it("discards results when the document changes during initialization", async () => {
    const source = document("Value:string");
    const { provider, showErrorMessage } = activate(async (file) => {
      source.version++;
      return readFile(file);
    });
    const edits = await provider.provideDocumentFormattingEdits(
      source,
      {},
      { isCancellationRequested: false },
    );
    expect(edits).toHaveLength(0);
    expect(showErrorMessage).not.toHaveBeenCalled();
  });

  it("discards results cancelled during initialization", async () => {
    const cancellation = { isCancellationRequested: false };
    const { provider, showErrorMessage } = activate(async (file) => {
      cancellation.isCancellationRequested = true;
      return readFile(file);
    });
    const edits = await provider.provideDocumentFormattingEdits(
      document("Value:string"),
      {},
      cancellation,
    );
    expect(edits).toHaveLength(0);
    expect(showErrorMessage).not.toHaveBeenCalled();
  });

  it("can retry after a Wasm loading error", async () => {
    let fail = true;
    const { provider, showErrorMessage } = activate(async (file) => {
      if (fail) throw new Error("Cannot read Wasm");
      return readFile(file);
    });
    const source = document("Value:string");
    const failed = await provider.provideDocumentFormattingEdits(
      source,
      {},
      { isCancellationRequested: false },
    );
    expect(failed).toHaveLength(0);
    assert.match(showErrorMessage.mock.calls[0][0], /Cannot read Wasm/);

    fail = false;
    const edits = await provider.provideDocumentFormattingEdits(
      source,
      {},
      { isCancellationRequested: false },
    );
    expect(edits[0].newText).toBe("Value: string\n");
  });
});

type Position = Pick<vscode.Position, "line" | "character">;

type Uri = Pick<vscode.Uri, "fsPath">;

interface FormattingProvider {
  provideDocumentFormattingEdits(
    document: Pick<vscode.TextDocument, "version" | "isClosed" | "getText"> & {
      positionAt(offset: number): Position;
    },
    options: Partial<vscode.FormattingOptions>,
    cancellation: Pick<vscode.CancellationToken, "isCancellationRequested">,
  ): Promise<vscode.TextEdit[]>;
}

// Exercise formatting in the shipped bundle independently of the language server.
function activate(readWasm: (file: string) => Promise<Uint8Array> = readFile) {
  let provider: FormattingProvider | undefined;
  let selector: vscode.DocumentFilter | undefined;
  const subscriptions: vscode.Disposable[] = [];
  const read = vi.fn((uri: Uri) => readWasm(uri.fsPath));
  const showErrorMessage = vi.fn((_message: string) => {});
  const vscode = {
    ...Object.fromEntries(
      [
        "CompletionItem",
        "CodeLens",
        "DocumentLink",
        "CodeAction",
        "Diagnostic",
        "CallHierarchyItem",
        "TypeHierarchyItem",
        "SymbolInformation",
        "InlayHint",
        "CancellationError",
      ].map((name) => [name, class {}]),
    ),
    languages: {
      registerDocumentFormattingEditProvider(
        language: vscode.DocumentFilter,
        formatter: FormattingProvider,
      ) {
        selector = language;
        provider = formatter;
        return { dispose() {} };
      },
    },
    workspace: { fs: { readFile: read } },
    window: { showErrorMessage },
    Uri: {
      file: (fsPath: string): Uri => ({ fsPath }),
      joinPath: (base: Uri, file: string): Uri => ({ fsPath: path.join(base.fsPath, file) }),
    },
    Range: class {
      start: Position;
      end: Position;

      constructor(start: Position, end: Position) {
        this.start = start;
        this.end = end;
      }
    },
    TextEdit: {
      replace: (range: { start: Position; end: Position }, newText: string) => ({ range, newText }),
    },
  };
  const dirname = path.resolve(__dirname, "../../dist/production");
  const module: {
    exports: {
      activateFormatting?: (context: Pick<vscode.ExtensionContext, "subscriptions">) => void;
    };
  } = { exports: {} };
  vm.runInNewContext(readFileSync(path.join(dirname, "extension.js"), "utf8"), {
    module,
    exports: module.exports,
    __dirname: dirname,
    require(name: string) {
      if (isBuiltin(name)) return createRequire(__filename)(name);
      expect(name, "the bundled extension must not require an LSP or workspace package").toBe(
        "vscode",
      );
      return vscode;
    },
    TextEncoder,
    TextDecoder,
    WebAssembly,
    console,
    process,
    Buffer,
    setTimeout,
    clearTimeout,
    setImmediate,
    clearImmediate,
  });
  assert.ok(module.exports.activateFormatting);
  module.exports.activateFormatting({ subscriptions });
  assert.ok(provider);
  assert.ok(selector);
  return { provider, selector, subscriptions, read, showErrorMessage };
}

function document(source: string) {
  return {
    version: 1,
    isClosed: false,
    getText: () => source,
    positionAt(offset: number): Position {
      const lines = source.slice(0, offset).split("\n");
      return { line: lines.length - 1, character: lines[lines.length - 1].length };
    },
  };
}
