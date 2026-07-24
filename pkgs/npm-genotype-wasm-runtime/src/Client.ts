import * as Gt from "@genotype-lang/types";
import { GtwmFs } from "./Fs";
import { GtwmCompilerRpc } from "./compiler/CompilerRpc";
import { GtwmFormattersRpc } from "./formatters/FormattersRpc";
import { RpcWorkerClientTransport } from "@js-fns/rpc/transports/worker";

export namespace GtwmClient {
  export interface Props {
    fs: GtwmFs;
    cwdPath?: string;
    basePath?: string;
    onDiagnostic?: (diagnostic: Gt.GtDiagnostic) => void;
  }

  export type OnDiagnostic = (diagnostic: Gt.GtDiagnostic) => void;

  export type Formatter = FormatterTs | FormatterRs | FormatterPy;

  export interface FormatterTs {
    kind: "oxfmt";
    extension: ".ts";
  }

  export interface FormatterRs {
    kind: "prettyplease";
    extension: ".rs";
  }

  export interface FormatterPy {
    kind: "ruff";
    extension: ".py";
  }
}

export class GtwmClient {
  #fs: GtwmFs;
  #onDiagnostic: GtwmClient.OnDiagnostic | undefined;
  #compilerWorker: Worker | undefined;
  #formattersWorker: Worker | undefined;
  #compilerPeerPromise: Promise<GtwmCompilerRpc.ClientPeer>;
  #formattersPeer: GtwmFormattersRpc.ClientPeer;

  constructor(props: GtwmClient.Props) {
    const { fs, onDiagnostic } = props;
    this.#fs = fs;
    this.#onDiagnostic = onDiagnostic;

    const { cwdPath = "/workspace", basePath = "." } = props;
    this.#compilerPeerPromise = this.#initCompilerWorker({ cwdPath, basePath });

    this.#formattersPeer = this.#initFormattersWorker();
  }

  #initCompilerWorker(request: GtwmCompilerRpc.InitRequest): Promise<GtwmCompilerRpc.ClientPeer> {
    const worker = new Worker(new URL("./compiler/CompilerWorker.ts", import.meta.url), {
      type: "module",
    });
    this.#compilerWorker = worker;

    const compilerPeer = GtwmCompilerRpc.rpc.peer("client", new RpcWorkerClientTransport(worker), {
      "report-diagnostic": this.#onReportDiagnostic.bind(this),

      "file-exists": this.#onFileExists.bind(this),

      "read-file": this.#onReadFile.bind(this),

      "write-file": this.#onWriteFile.bind(this),

      "find-file": this.#onFindFile.bind(this),

      "glob-files": this.#onGlobFiles.bind(this),

      "is-file": this.#onIsFile.bind(this),

      "run-formatter": this.#onRunFormatter.bind(this),
    });

    return compilerPeer.call("init", request).then(() => compilerPeer);
  }

  #initFormattersWorker(): GtwmFormattersRpc.ClientPeer {
    const worker = new Worker(new URL("./formatters/FormattersWorker.ts", import.meta.url), {
      type: "module",
    });
    this.#formattersWorker = worker;

    return GtwmFormattersRpc.rpc.peer("client", new RpcWorkerClientTransport(worker), {});
  }

  dispose(): void {
    this.#compilerWorker?.terminate();
    this.#compilerWorker = undefined;

    this.#formattersWorker?.terminate();
    this.#formattersWorker = undefined;
  }

  async loadInProject(): Promise<Gt.GtcMetaLoadedProject> {
    const peer = await this.#compilerPeerPromise;
    const response = await peer.call("load-in-project", {
      kind: "load-in-project",
    });
    return response.meta;
  }

  async loadInModules(): Promise<Gt.GtcMetaLoadedModules> {
    const peer = await this.#compilerPeerPromise;
    const response = await peer.call("load-in-modules", { kind: "load-in-modules" });
    return response.meta;
  }

  async compile(): Promise<Gt.GtcMetaCompiled> {
    const peer = await this.#compilerPeerPromise;
    const response = await peer.call("compile", { kind: "compile" });
    return response.meta;
  }

  async #onReportDiagnostic(
    request: Gt.GtbRemoteBackendRequestReportDiagnostic,
  ): Promise<Gt.GtbRemoteBackendRequestResponseReportDiagnostic> {
    this.#onDiagnostic?.(request.diagnostic);
    return { kind: "report-diagnostic" };
  }

  async #onFileExists(
    request: Gt.GtbRemoteBackendRequestFileExists,
  ): Promise<Gt.GtbRemoteBackendRequestResponseFileExists> {
    const exists = this.#fs.isFile(request.path);
    return { kind: "file-exists", exists };
  }

  async #onReadFile(
    request: Gt.GtbRemoteBackendRequestReadFile,
  ): Promise<Gt.GtbRemoteBackendRequestResponseReadFile> {
    const content = this.#fs.readFile(request.path);
    if (content === null) throw new Error(`File '${request.path}' does not exist`);
    return { kind: "read-file", content };
  }

  async #onWriteFile(
    request: Gt.GtbRemoteBackendRequestWriteFile,
  ): Promise<Gt.GtbRemoteBackendRequestResponseWriteFile> {
    this.#fs.writeFile(request.path, request.content);
    return { kind: "write-file" };
  }

  async #onFindFile(
    request: Gt.GtbRemoteBackendRequestFindFile,
  ): Promise<Gt.GtbRemoteBackendRequestResponseFindFile> {
    const path = this.#fs.findFile(request.fileName);
    if (path === null) throw new Error(`Could not find '${request.fileName}'`);
    return { kind: "find-file", path };
  }

  async #onGlobFiles(
    request: Gt.GtbRemoteBackendRequestGlobFiles,
  ): Promise<Gt.GtbRemoteBackendRequestResponseGlobFiles> {
    const paths = this.#fs.glob(request.path);
    return { kind: "glob-files", paths };
  }

  async #onIsFile(
    request: Gt.GtbRemoteBackendRequestIsFile,
  ): Promise<Gt.GtbRemoteBackendRequestResponseIsFile> {
    const isFile = this.#fs.isFile(request.path);
    return { kind: "is-file", isFile };
  }

  async #onRunFormatter(
    request: Gt.GtbRemoteBackendRequestRunFormatter,
  ): Promise<Gt.GtbRemoteBackendRequestResponseRunFormatter> {
    const formatter = this.#resolveFormatter(request);
    if (!formatter) return { kind: "run-formatter" };

    await Promise.all(
      this.#fs.listFiles().map(async (filePath) => {
        if (!filePath.endsWith(formatter.extension)) return;

        const content = this.#fs.readFile(filePath);
        if (content === null) return;

        try {
          const { content: formatted } = await this.#formattersPeer.call("format", {
            kind: formatter.kind,
            content,
          });
          this.#fs.writeFile(filePath, formatted);
        } catch (error) {
          this.#onDiagnostic?.({
            kind: "error",
            content: {
              title: `Failed to write formatted content to file '${filePath}'`,
              body: String(error),
            },
          });
        }
      }),
    );

    return { kind: "run-formatter" };
  }

  #resolveFormatter(request: Gt.GtbRemoteBackendRequestRunFormatter): GtwmClient.Formatter | null {
    switch (request.formatter.kind) {
      case "oxfmt":
        return { kind: "oxfmt", extension: ".ts" };

      case "ruff":
        return { kind: "ruff", extension: ".py" };

      case "prettyplease":
        return { kind: "prettyplease", extension: ".rs" };
    }

    return null;
  }
}
