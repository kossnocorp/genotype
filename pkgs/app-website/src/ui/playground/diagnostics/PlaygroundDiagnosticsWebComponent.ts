import flatPromise from "../../utils/promise";
import { Wc } from "../../utils/Wc";
import { PlaygroundDiagnosticsManager } from "./PlaygroundDiagnosticsManager";

export class PlaygroundDiagnosticsWebComponent extends Wc {
  static readonly tag = "gt-playground-diagnostics";

  static register(): void {
    if (customElements.get(this.tag)) return;
    customElements.define(this.tag, this);
  }

  #managerFlatPromise = flatPromise<PlaygroundDiagnosticsManager>();

  connectedCallback(): void {
    this.#managerFlatPromise.resolve(new PlaygroundDiagnosticsManager({ wc: this }));
  }

  clear(): void {
    this.locateAll("[data-diagnostic]").forEach((diagnosticEl) => diagnosticEl.remove());
    this.scrollTop = 0;
  }

  get managerPromise(): Promise<PlaygroundDiagnosticsManager> {
    return this.#managerFlatPromise.promise;
  }
}
