import { z } from "zod";
import { GtwmMessage } from "./Message";
import { always } from "alwaysly";

export namespace GtwmMessenger {
  //#region Schema

  export interface PayloadSchema<
    InboundRequest extends {},
    InboundRequestResponse extends {},
    OutboundRequest extends {},
    OutboundRequestResponse extends {},
  > {
    InboundRequest: z.ZodType<InboundRequest>;
    InboundRequestResponse: z.ZodType<InboundRequestResponse>;
    OutboundRequest: z.ZodType<OutboundRequest>;
    OutboundRequestResponse: z.ZodType<OutboundRequestResponse>;
  }

  //#endregion

  //#region Payload

  export type PayloadRequest<Schema extends PayloadSchema<any, any, any, any>> = z.infer<
    Schema["OutboundRequest"]
  >;

  export type PayloadRequestResponse<Schema extends PayloadSchema<any, any, any, any>> = z.infer<
    Schema["OutboundRequestResponse"]
  >;

  export type PayloadRequestResponseFor<
    Schema extends PayloadSchema<any, any, any, any>,
    Request extends PayloadRequest<Schema>,
  > = PayloadRequestResponse<Schema> & { kind: Request["kind"] };

  //#endregion

  //#region Message

  export type MessageInboundRequest<Schema extends PayloadSchema<any, any, any, any>> =
    GtwmMessage.Request<z.infer<Schema["InboundRequest"]>>;

  export type MessageInboundRequestResponse<Schema extends PayloadSchema<any, any, any, any>> =
    GtwmMessage.Response<z.infer<Schema["InboundRequestResponse"]>>;

  export type MessageOutboundRequestResponse<Schema extends PayloadSchema<any, any, any, any>> =
    GtwmMessage.Response<z.infer<Schema["OutboundRequestResponse"]>>;

  export type MessageOutboundRequest<Schema extends PayloadSchema<any, any, any, any>> =
    GtwmMessage.Request<z.infer<Schema["OutboundRequest"]>>;

  export type MessageRecipientPostable<Schema extends PayloadSchema<any, any, any, any>> =
    | MessageInboundRequest<Schema>
    | MessageOutboundRequestResponse<Schema>;

  export type MessageRecipientOnable<Schema extends PayloadSchema<any, any, any, any>> =
    | MessageInboundRequestResponse<Schema>
    | MessageOutboundRequest<Schema>;

  //#endregion

  //#region Routing

  export interface OutboundRequestResponseResolver<
    Schema extends PayloadSchema<any, any, any, any>,
  > {
    resolve: (value: z.infer<Schema["OutboundRequestResponse"]>) => void;
    reject: (reason?: unknown) => void;
  }

  export type InboundRequestHandler<Schema extends PayloadSchema<any, any, any, any>> = <
    Request extends z.infer<Schema["InboundRequest"]>,
  >(
    inboundRequest: z.infer<Schema["InboundRequest"]>,
  ) => Promise<z.infer<Schema["InboundRequestResponse"]> & { kind: Request["kind"] }>;

  //#endregion

  //#region Recipient

  export type MessageConstraint<
    RequestPayload extends {} = any,
    ResponsePayload extends {} = any,
  > = GtwmMessage.Request<RequestPayload> | GtwmMessage.Response<ResponsePayload>;

  export interface Recipient<Schema extends PayloadSchema<any, any, any, any>> {
    Schema: Schema;

    post(message: MessageRecipientPostable<Schema>): void;

    on(handler: RecipientOnHandler<Schema>): RecipientOff;
  }

  export type RecipientOnHandler<Schema extends PayloadSchema<any, any, any, any>> = (
    message: MessageRecipientOnable<Schema>,
  ) => void | Promise<void>;

  export type RecipientOff = () => void;

  //#endregion
}

export class GtwmMessenger<Recipient extends GtwmMessenger.Recipient<any>> {
  static MessageRecipientOnable = <Schema extends GtwmMessenger.PayloadSchema<any, any, any, any>>(
    Schema: Schema,
  ) =>
    z.union([
      GtwmMessage.Response(Schema.OutboundRequestResponse),
      GtwmMessage.Request(Schema.InboundRequest),
    ]);

  #recipient: Recipient;
  #recipientOff: GtwmMessenger.RecipientOff;

  #responseResolvers: Record<
    string,
    GtwmMessenger.OutboundRequestResponseResolver<Recipient["Schema"]>
  > = {};
  #inboundRequestHandler: GtwmMessenger.InboundRequestHandler<Recipient["Schema"]>;

  constructor(
    recipient: Recipient,
    inboundRequestHandler: GtwmMessenger.InboundRequestHandler<Recipient["Schema"]>,
  ) {
    this.#recipient = recipient;
    this.#inboundRequestHandler = inboundRequestHandler;

    this.#recipientOff = this.#recipient.on(this.#onInboundMessage.bind(this));
  }

  off() {
    this.#recipientOff();
  }

  request<Request extends GtwmMessenger.PayloadRequest<Recipient["Schema"]>>(
    request: Request,
  ): Promise<GtwmMessenger.PayloadRequestResponseFor<Recipient["Schema"], Request>> {
    const id = crypto.randomUUID();
    return new Promise((resolve, reject) => {
      this.#responseResolvers[id] = { resolve, reject };
      this.#recipient.post({ id, kind: "request", payload: request });
    });
  }

  async #onInboundMessage(message: GtwmMessenger.MessageRecipientOnable<any>): Promise<void> {
    const MessageInbound = z.union([
      GtwmMessage.Request(this.#recipient.Schema.InboundRequest),
      GtwmMessage.Response(this.#recipient.Schema.OutboundRequestResponse),
    ]);

    const parsed = MessageInbound.parse(message);

    switch (parsed.kind) {
      case "request":
        return this.#onInboundRequest(parsed);

      case "response":
        return this.#onOutboundRequestResponse(parsed);

      default:
        parsed satisfies never;
    }
  }

  #onOutboundRequestResponse(
    message: GtwmMessenger.MessageOutboundRequestResponse<Recipient["Schema"]>,
  ): void {
    const resolver = this.#responseResolvers[message.id];
    always(resolver);
    const { resolve, reject } = resolver;

    if (message.status === "error") reject(new Error(message.error));
    else
      // NOTE: Zod fails to properly infer generic types.
      resolve(message.payload as z.infer<Recipient["Schema"]["OutboundRequestResponse"]>);

    delete this.#responseResolvers[message.id];
  }

  async #onInboundRequest(message: GtwmMessenger.MessageInboundRequest<Recipient["Schema"]>) {
    const { id, payload } = message;

    try {
      // NOTE: Zod fails to properly infer generic types.
      const response = await this.#inboundRequestHandler(
        payload as z.infer<Recipient["Schema"]["InboundRequest"]>,
      );
      this.#recipient.post({
        kind: "response",
        id,
        status: "ok",
        payload: response,
      });
    } catch (error) {
      this.#recipient.post({
        kind: "response",
        id,
        status: "error",
        error: String(error),
      });
    }
  }
}
