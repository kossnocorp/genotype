import * as Gt from "@genotype-lang/types";
import initWasm, { GtwmCompiler } from "@genotype-lang/wasm";
import type { z } from "zod";
import { GtwmCompilerRpc } from "./CompilerRpc";
import { flatPromise, type FlatPromise } from "@js-fns/promise";
import { RpcWorkerServerTransport } from "@js-fns/rpc/transports/worker";

// NOTE: This class is initiated right below, hence never exported.

class GtwmCompilerWorker {
  #peer: GtwmCompilerRpc.WorkerPeer;

  #compilerPromise: Promise<GtwmCompiler>;
  #compilerResolver: FlatPromise.Resolver<GtwmCompiler> = {
    resolve: () => {},
    reject: () => {},
  };

  constructor() {
    this.#peer = GtwmCompilerRpc.rpc.peer("worker", new RpcWorkerServerTransport(), {
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

  async #onInit(input: GtwmCompilerRpc.InitRequest): Promise<Record<string, never>> {
    const { cwdPath, basePath } = input;
    await initWasm();
    const compiler = new GtwmCompiler(cwdPath, basePath, (request: Gt.GtbRemoteBackendRequest) =>
      this.#peer.call(request.kind, request),
    );
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
}

new GtwmCompilerWorker();
