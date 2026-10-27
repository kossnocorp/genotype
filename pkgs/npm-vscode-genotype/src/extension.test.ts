import { beforeEach, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  start: vi.fn(() => Promise.resolve()),
  stop: vi.fn(() => Promise.resolve()),
  client: vi.fn(),
  get: vi.fn(),
  watch: vi.fn(() => ({ dispose: vi.fn() })),
  register: vi.fn(() => ({ dispose: vi.fn() })),
  error: vi.fn(),
  command: vi.fn(() => ({ dispose: vi.fn() })),
  actions: vi.fn(() => ({ dispose: vi.fn() })),
}));

vi.mock("vscode", () => ({
  workspace: {
    getConfiguration: () => ({ get: mocks.get }),
    createFileSystemWatcher: mocks.watch,
  },
  languages: {
    registerDocumentFormattingEditProvider: mocks.register,
    registerCodeActionsProvider: mocks.actions,
  },
  commands: { registerCommand: mocks.command },
  CodeActionKind: { RefactorRewrite: "refactor.rewrite" },
  window: { showErrorMessage: mocks.error },
}));
vi.mock("./formatter", () => ({ GenotypeFormattingProvider: class {} }));
vi.mock("vscode-languageclient/node", () => ({
  LanguageClient: class {
    constructor(...args: unknown[]) {
      mocks.client(...args);
    }
    start = mocks.start;
    stop = mocks.stop;
    dispose() {}
  },
}));

import { activate, deactivate } from "./extension";
import type { ExtensionContext } from "vscode";

beforeEach(() => {
  vi.clearAllMocks();
  mocks.get.mockReturnValue(undefined);
});

it("starts gt lsp and connects file synchronization alongside formatting", async () => {
  const context = { subscriptions: [] } as unknown as ExtensionContext;
  activate(context);
  expect(mocks.client).toHaveBeenCalledWith(
    "genotype",
    "Genotype",
    { command: "gt", args: ["lsp"] },
    {
      documentSelector: [{ scheme: "file", language: "genotype" }],
      synchronize: { fileEvents: mocks.watch.mock.results[0].value },
      middleware: { provideRenameEdits: expect.any(Function) },
    },
  );
  expect(mocks.start).toHaveBeenCalledOnce();
  expect(mocks.register).toHaveBeenCalledOnce();
  expect(context.subscriptions).toHaveLength(5);
  expect(mocks.command).toHaveBeenCalledWith(
    "genotype.renameAcrossLanguages",
    expect.any(Function),
  );
  await deactivate();
  expect(mocks.stop).toHaveBeenCalledOnce();
});

it("supports a custom CLI executable", () => {
  mocks.get.mockReturnValue({ command: "/custom/gt" });
  activate({ subscriptions: [] } as unknown as ExtensionContext);
  expect(mocks.client.mock.calls[0][2]).toEqual({ command: "/custom/gt", args: ["lsp"] });
});

it("reports startup failure while retaining formatting", async () => {
  mocks.start.mockRejectedValueOnce(new Error("ENOENT"));
  activate({ subscriptions: [] } as unknown as ExtensionContext);
  await Promise.resolve();
  expect(mocks.error).toHaveBeenCalledWith(expect.stringContaining("gt is installed and on PATH"));
  expect(mocks.register).toHaveBeenCalledOnce();
});
