import * as Gt from "@genotype-lang/types";
import { GtwmWorkerClient } from "./worker/WorkerClient";
import { GtwmMessenger } from "./message/Messenger";
import { GtwmFs } from "./Fs";

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
  #workerClient: GtwmWorkerClient;
  #messengerPromise: Promise<GtwmMessenger<GtwmWorkerClient>>;

  constructor(props: GtwmClient.Props) {
    const { fs, cwdPath = "/workspace", basePath = ".", onDiagnostic } = props;

    this.#fs = fs;
    this.#onDiagnostic = onDiagnostic;

    this.#workerClient = new GtwmWorkerClient();
    const messenger = new GtwmMessenger(this.#workerClient, this.#onBackendRequest.bind(this));

    this.#messengerPromise = messenger
      .request({ kind: "init", cwdPath, basePath })
      .then(() => messenger);
  }

  async compile(): Promise<number> {
    const messenger = await this.#messengerPromise;
    try {
      await messenger.request({ kind: "load-in-project" });
      await messenger.request({ kind: "load-in-modules" });
      const response = await messenger.request({ kind: "compile" });
      return response.exitCode;
    } catch (error) {
      return 1;
    }
  }

  async #onBackendRequest(
    request: Gt.GtbRemoteBackendRequest,
  ): Promise<Gt.GtbRemoteBackendRequestResponse> {
    switch (request.kind) {
      case "report-diagnostic":
        return this.#onReportDiagnostic(request);

      case "file-exists":
        return this.#onFileExists(request);

      case "read-file":
        return this.#onReadFile(request);

      case "write-file":
        return this.#onWriteFile(request);

      case "find-file":
        return this.#onFindFile(request);

      case "glob-files":
        return this.#onGlobFiles(request);

      case "is-file":
        return this.#onIsFile(request);

      case "run-formatter":
        return this.#onRunFormatter(request);
    }
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
    return {
      kind: "file-exists",
      exists,
    };
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
    return { kind: "run-formatter" };
  }
}
