import { GtwmMessage } from "../message/Message";
import { GtwmMessenger } from "../message/Messenger";
import * as Gt from "@genotype-lang/types";

export class GtwmWorkerClient implements GtwmMessenger.Recipient<typeof GtwmWorkerClient.Schema> {
  static Schema = {
    InboundRequest: Gt.GtbRemoteBackendRequest,
    InboundRequestResponse: Gt.GtbRemoteBackendRequestResponse,
    OutboundRequest: GtwmMessage.ClientRequest,
    OutboundRequestResponse: GtwmMessage.ClientRequestResponse,
  };

  #worker: Worker;

  constructor() {
    this.#worker = new Worker(new URL("./Worker.ts", import.meta.url), { type: "module" });
  }

  get Schema() {
    return GtwmWorkerClient.Schema;
  }

  post(message: GtwmMessenger.MessageRecipientPostable<typeof GtwmWorkerClient.Schema>) {
    this.#worker.postMessage(message);
  }

  on(
    handler: GtwmMessenger.RecipientOnHandler<typeof GtwmWorkerClient.Schema>,
  ): GtwmMessenger.RecipientOff {
    const handlerWrapper = (
      event: MessageEvent<GtwmMessenger.MessageRecipientOnable<typeof GtwmWorkerClient.Schema>>,
    ) => {
      const Message = GtwmMessenger.MessageRecipientOnable(GtwmWorkerClient.Schema);
      const message = Message.parse(event.data);
      handler(message);
    };

    this.#worker.addEventListener("message", handlerWrapper);
    return () => this.#worker.removeEventListener("message", handlerWrapper);
  }
}
