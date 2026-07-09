import { GtwmMessage } from "../message/Message";
import * as Gt from "@genotype-lang/types";
import { GtwmMessenger } from "../message/Messenger";

export class GtwmWorkerServer implements GtwmMessenger.Recipient<typeof GtwmWorkerServer.Schema> {
  static Schema = {
    InboundRequest: GtwmMessage.ClientRequest,
    InboundRequestResponse: GtwmMessage.ClientRequestResponse,
    OutboundRequest: Gt.GtbRemoteBackendRequest,
    OutboundRequestResponse: Gt.GtbRemoteBackendRequestResponse,
  };

  get Schema() {
    return GtwmWorkerServer.Schema;
  }

  post(message: GtwmMessenger.MessageRecipientPostable<typeof GtwmWorkerServer.Schema>) {
    self.postMessage(message);
  }

  on(handler: GtwmMessenger.RecipientOnHandler<typeof this.Schema>): GtwmMessenger.RecipientOff {
    const handlerWrapper = (
      event: MessageEvent<GtwmMessenger.MessageRecipientOnable<typeof GtwmWorkerServer.Schema>>,
    ) => {
      const Message = GtwmMessenger.MessageRecipientOnable(GtwmWorkerServer.Schema);
      const message = Message.parse(event.data);
      handler(message);
    };

    self.addEventListener("message", handlerWrapper);
    return () => self.removeEventListener("message", handlerWrapper);
  }
}
