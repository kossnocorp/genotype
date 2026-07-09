import * as Gt from "@genotype-lang/types";
import { z } from "zod";

export namespace GtwmMessage {
  //#region Base

  export type Request<Payload extends {}> = z.infer<
    ReturnType<typeof GtwmMessage.Request<z.ZodObject<Payload>>>
  >;

  export type ResponseOk<Payload extends {}> = z.infer<
    ReturnType<typeof GtwmMessage.ResponseOk<z.ZodObject<Payload>>>
  >;

  export type ResponseError<Payload extends {}> = z.infer<
    ReturnType<typeof GtwmMessage.ResponseError<z.ZodObject<Payload>>>
  >;

  export type Response<Payload extends {}> = z.infer<
    ReturnType<typeof GtwmMessage.Response<z.ZodObject<Payload>>>
  >;

  //#endregion

  //#region Client

  export type ClientRequestInit = z.infer<typeof GtwmMessage.ClientRequestInit>;

  export type ClientRequestResponseInit = z.infer<typeof GtwmMessage.ClientRequestResponseInit>;

  export type ClientRequest = z.infer<typeof GtwmMessage.ClientRequest>;

  export type ClientRequestResponse = z.infer<typeof GtwmMessage.ClientRequestResponse>;

  //#endregion
}

export class GtwmMessage {
  //#region Base

  static #Base = z.object({
    id: z.string(),
  });

  static Request = <Payload extends z.ZodType>(Payload: Payload) =>
    this.#Base.extend({
      kind: z.literal("request"),
      payload: Payload,
    });

  static #ResponseBase = this.#Base.extend({
    kind: z.literal("response"),
  });

  static ResponseOk = <Payload extends z.ZodType>(Payload: Payload) =>
    this.#ResponseBase.extend({
      status: z.literal("ok"),
      payload: Payload,
    });

  static ResponseError = <Payload extends z.ZodType>(Payload: Payload) =>
    this.#ResponseBase.extend({
      status: z.literal("error"),
      error: z.string(),
    });

  static Response = <Payload extends z.ZodType>(Payload: Payload) =>
    z.union([this.ResponseOk(Payload), this.ResponseError(Payload)]);

  //#endregion

  //#region Client

  static ClientRequestInit = z.object({
    kind: z.literal("init"),
    cwdPath: z.string(),
    basePath: z.string(),
  });

  static ClientRequestResponseInit = z.object({
    kind: z.literal("init"),
  });

  static ClientRequest = z.union([this.ClientRequestInit, Gt.GtbRemoteRuntimeRequest]);

  static ClientRequestResponse = z.union([
    this.ClientRequestResponseInit,
    Gt.GtbRemoteRuntimeRequestResponse,
  ]);

  //#endregion
}
