import * as Gt from "@genotype-lang/types";
import { GtwmFs } from "./Fs";
import { GtwmRpc } from "./rpc";
import { RpcWorkerClientTransport } from "@js-fns/rpc/transports/worker";

export namespace GtwmClient {
  export interface Props {
    fs: GtwmFs;
    cwdPath?: string;
    basePath?: string;
    onDiagnostic?: (diagnostic: Gt.GtDiagnostic) => void;
  }

  export type OnDiagnostic = (diagnostic: Gt.GtDiagnostic) => void;
}

export class GtwmClient {
  #fs: GtwmFs;
  #onDiagnostic: GtwmClient.OnDiagnostic | undefined;
  #peerPromise: Promise<GtwmRpc.ClientPeer>;
  #worker: Worker;

  constructor(props: GtwmClient.Props) {
    const { fs, cwdPath = "/workspace", basePath = ".", onDiagnostic } = props;

    this.#fs = fs;
    this.#onDiagnostic = onDiagnostic;

    this.#worker = new Worker(new URL("./Worker.ts", import.meta.url), {
      type: "module",
    });

    const peer = GtwmRpc.rpc.peer("client", new RpcWorkerClientTransport(this.#worker), {
      "report-diagnostic": this.#onReportDiagnostic.bind(this),

      "file-exists": this.#onFileExists.bind(this),

      "read-file": this.#onReadFile.bind(this),

      "write-file": this.#onWriteFile.bind(this),

      "find-file": this.#onFindFile.bind(this),

      "glob-files": this.#onGlobFiles.bind(this),

      "is-file": this.#onIsFile.bind(this),

      "run-formatter": this.#onRunFormatter.bind(this),
    });

    this.#peerPromise = peer.call("init", { cwdPath, basePath }).then(() => peer);
  }

  async loadInProject(): Promise<Gt.GtcMetaLoadedProject> {
    const peer = await this.#peerPromise;
    const response = await peer.call("load-in-project", {
      kind: "load-in-project",
    });
    return response.meta;
  }

  async loadInModules(): Promise<Gt.GtcMetaLoadedModules> {
    const peer = await this.#peerPromise;
    const response = await peer.call("load-in-modules", {
      kind: "load-in-modules",
    });
    return response.meta;
  }

  async compile(): Promise<Gt.GtcMetaCompiled> {
    const peer = await this.#peerPromise;
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
    _request: Gt.GtbRemoteBackendRequestRunFormatter,
  ): Promise<Gt.GtbRemoteBackendRequestResponseRunFormatter> {
    return { kind: "run-formatter" };
  }
}
