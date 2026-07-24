import initFormattersWasm, { GtwmFormatters } from "@genotype-lang/formatters-wasm";
import { RpcWorkerServerTransport } from "@js-fns/rpc/transports/worker";
import { GtwmFormattersRpc } from "./FormattersRpc";

// NOTE: This class is initiated right below, hence never exported.

class GtwmFormattersWorker {
  #formattersPromise = initFormattersWasm().then(() => new GtwmFormatters());

  constructor() {
    GtwmFormattersRpc.rpc.peer("worker", new RpcWorkerServerTransport(), {
      format: this.#onFormat.bind(this),
    });
  }

  async #onFormat(
    request: GtwmFormattersRpc.FormatRequest,
  ): Promise<GtwmFormattersRpc.FormatResponse> {
    const content = await this.#formatContent(request);
    return { content };
  }

  async #formatContent(request: GtwmFormattersRpc.FormatRequest): Promise<string> {
    const formatters = await this.#formattersPromise;
    const { kind, content } = request;

    switch (kind) {
      case "oxfmt":
        return formatters.formatTs(content);

      case "ruff":
        return formatters.formatPy(content);

      case "prettyplease":
        return formatters.formatRs(content);
    }
  }
}

new GtwmFormattersWorker();
