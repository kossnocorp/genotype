import * as Gt from "@genotype-lang/types";
import initWasm, { GtwmCompiler } from "@genotype-lang/wasm";
import type { z } from "zod";
import { GtwmRpc } from "./rpc";
import flatPromise, { FlatPromise } from "./utils/promise";
import { RpcWorkerServerTransport } from "@js-fns/rpc/transports/worker";

// NOTE: This class is initiated right below, hence never exported.

class GtwmWorker {
  #peer: GtwmRpc.WorkerPeer;

  #compilerPromise: Promise<GtwmCompiler>;
  #compilerResolver: FlatPromise.Resolver<GtwmCompiler> = {
    resolve: () => {},
    reject: () => {},
  };

  constructor() {
    this.#peer = GtwmRpc.rpc.peer("worker", new RpcWorkerServerTransport(), {
      init: this.#onInit.bind(this),

      "load-in-project": (request) =>
        this.#onRuntimeRequest(request, Gt.GtcRemoteRuntimeRequestResponseLoadInProject),

      "load-in-modules": (request) =>
        this.#onRuntimeRequest(request, Gt.GtcRemoteRuntimeRequestResponseLoadInModules),

      compile: (request) =>
        this.#onRuntimeRequest(request, Gt.GtcRemoteRuntimeRequestResponseCompile),
    });

    const { promise, ...compilerResolver } = flatPromise<GtwmCompiler>();
    this.#compilerPromise = promise;
    this.#compilerResolver = compilerResolver;
  }

  async #onInit(input: { cwdPath: string; basePath: string }): Promise<Record<string, never>> {
    const { cwdPath, basePath } = input;
    await initWasm();
    const compiler = new GtwmCompiler(cwdPath, basePath, this.#onBackendRequest.bind(this));
    this.#compilerResolver.resolve(compiler);
    return {};
  }

  async #onRuntimeRequest<Response extends Gt.GtcRemoteRuntimeRequestResponse>(
    request: Gt.GtcRemoteRuntimeRequest,
    responseSchema: z.ZodType<Response>,
  ): Promise<Response> {
    const compiler = await this.#compilerPromise;
    const rawResponse = await compiler.handleRuntimeRequest(request);
    return responseSchema.parse(rawResponse);
  }

  async #onBackendRequest(
    request: Gt.GtbRemoteBackendRequest,
  ): Promise<Gt.GtbRemoteBackendRequestResponse> {
    switch (request.kind) {
      case "glob-files":
        return this.#peer.call("glob-files", request);

      case "read-file":
        return this.#peer.call("read-file", request);

      case "file-exists":
        return this.#peer.call("file-exists", request);

      case "is-file":
        return this.#peer.call("is-file", request);

      case "find-file":
        return this.#peer.call("find-file", request);

      case "report-diagnostic":
        return this.#peer.call("report-diagnostic", request);

      case "run-formatter":
        return this.#peer.call("run-formatter", request);

      case "write-file":
        return this.#peer.call("write-file", request);
    }
  }
}

new GtwmWorker();
