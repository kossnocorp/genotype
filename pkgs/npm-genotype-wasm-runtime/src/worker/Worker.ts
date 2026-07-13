import initWasm, { GtwmCompiler } from "@genotype-lang/wasm";
import { GtwmWorkerServer } from "./WorkerServer";
import * as Gt from "@genotype-lang/types";
import { GtwmMessenger } from "../message/Messenger";
import { GtwmMessage } from "../message/Message";
import flatPromise, { FlatPromise } from "../utils/promise";

// NOTE: This class is initiated right below, hence never exported.

class GtwmWorker {
  #workerServer: GtwmWorkerServer;
  #messenger: GtwmMessenger<GtwmWorkerServer>;

  #compilerPromise: Promise<GtwmCompiler>;
  #compilerResolver: FlatPromise.Resolver<GtwmCompiler> = {
    resolve: () => {},
    reject: () => {},
  };

  constructor() {
    this.#workerServer = new GtwmWorkerServer();
    this.#messenger = new GtwmMessenger(this.#workerServer, this.#onClientRequest.bind(this));

    const { promise, ...compilerResolver } = flatPromise<GtwmCompiler>();
    this.#compilerPromise = promise;
    this.#compilerResolver = compilerResolver;
  }

  async #onClientRequest(
    request: GtwmMessage.ClientRequest,
  ): Promise<GtwmMessage.ClientRequestResponse> {
    if (request.kind === "init") return this.#onInit(request);

    return this.#onRuntimeRequest(request);
  }

  async #onInit(
    request: GtwmMessage.ClientRequestInit,
  ): Promise<GtwmMessage.ClientRequestResponseInit> {
    const { cwdPath, basePath } = request;
    await initWasm();
    const compiler = new GtwmCompiler(cwdPath, basePath, this.#onBackendRequest.bind(this));
    this.#compilerResolver.resolve(compiler);
    return { kind: "init" };
  }

  async #onRuntimeRequest(
    request: Gt.GtcRemoteRuntimeRequest,
  ): Promise<Gt.GtcRemoteRuntimeRequestResponse> {
    const compiler = await this.#compilerPromise;
    const rawResponse = await compiler.handleRuntimeRequest(request);
    return Gt.GtcRemoteRuntimeRequestResponse.parse(rawResponse);
  }

  async #onBackendRequest(
    request: Gt.GtbRemoteBackendRequest,
  ): Promise<Gt.GtbRemoteBackendRequestResponse> {
    return this.#messenger.request(request);
  }
}

new GtwmWorker();
