import { BiRpc, type BiRpcPeer } from "@js-fns/rpc/birpc";
import { z } from "zod";

export abstract class GtwmFormattersRpc {
  static FormatRequest = z.object({
    kind: z.enum(["oxfmt", "prettyplease", "ruff"]),
    content: z.string(),
  });

  static FormatResponse = z.object({
    content: z.string(),
  });

  static schema = {
    client: {
      format: {
        in: GtwmFormattersRpc.FormatRequest,

        out: GtwmFormattersRpc.FormatResponse,
      },
    },

    worker: {},
  };

  static rpc = new BiRpc(this.schema);
}

export namespace GtwmFormattersRpc {
  export type FormatRequest = z.infer<typeof GtwmFormattersRpc.FormatRequest>;

  export type FormatResponse = z.infer<typeof GtwmFormattersRpc.FormatResponse>;

  export type ClientPeer = BiRpcPeer<
    typeof GtwmFormattersRpc.schema.client,
    typeof GtwmFormattersRpc.schema.worker
  >;

  export type WorkerPeer = BiRpcPeer<
    typeof GtwmFormattersRpc.schema.worker,
    typeof GtwmFormattersRpc.schema.client
  >;
}
