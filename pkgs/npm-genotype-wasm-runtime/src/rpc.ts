import * as Gt from "@genotype-lang/types";
import { BiRpc, type BiRpcPeer } from "@js-fns/rpc/birpc";
import { z } from "zod";

export abstract class GtwmRpc {
  static schema = {
    client: {
      init: {
        in: z.object({
          cwdPath: z.string(),
          basePath: z.string(),
        }),
        out: z.object({}),
      },

      "load-in-project": {
        in: Gt.GtcRemoteRuntimeRequestLoadInProject,
        out: Gt.GtcRemoteRuntimeRequestResponseLoadInProject,
      },

      "load-in-modules": {
        in: Gt.GtcRemoteRuntimeRequestLoadInModules,
        out: Gt.GtcRemoteRuntimeRequestResponseLoadInModules,
      },

      compile: {
        in: Gt.GtcRemoteRuntimeRequestCompile,
        out: Gt.GtcRemoteRuntimeRequestResponseCompile,
      },
    },

    worker: {
      "glob-files": {
        in: Gt.GtbRemoteBackendRequestGlobFiles,
        out: Gt.GtbRemoteBackendRequestResponseGlobFiles,
      },

      "read-file": {
        in: Gt.GtbRemoteBackendRequestReadFile,
        out: Gt.GtbRemoteBackendRequestResponseReadFile,
      },

      "file-exists": {
        in: Gt.GtbRemoteBackendRequestFileExists,
        out: Gt.GtbRemoteBackendRequestResponseFileExists,
      },

      "is-file": {
        in: Gt.GtbRemoteBackendRequestIsFile,
        out: Gt.GtbRemoteBackendRequestResponseIsFile,
      },

      "find-file": {
        in: Gt.GtbRemoteBackendRequestFindFile,
        out: Gt.GtbRemoteBackendRequestResponseFindFile,
      },

      "report-diagnostic": {
        in: Gt.GtbRemoteBackendRequestReportDiagnostic,
        out: Gt.GtbRemoteBackendRequestResponseReportDiagnostic,
      },

      "run-formatter": {
        in: Gt.GtbRemoteBackendRequestRunFormatter,
        out: Gt.GtbRemoteBackendRequestResponseRunFormatter,
      },

      "write-file": {
        in: Gt.GtbRemoteBackendRequestWriteFile,
        out: Gt.GtbRemoteBackendRequestResponseWriteFile,
      },
    },
  };

  static rpc = new BiRpc(this.schema);
}

export namespace GtwmRpc {
  export type ClientPeer = BiRpcPeer<typeof GtwmRpc.schema.client, typeof GtwmRpc.schema.worker>;

  export type WorkerPeer = BiRpcPeer<typeof GtwmRpc.schema.worker, typeof GtwmRpc.schema.client>;
}
