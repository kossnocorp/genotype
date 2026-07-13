import flatPromise from "../utils/promise";
import { EditorManager } from "./EditorManager";

export namespace EditorWebComponent {}

export class EditorWebComponent extends HTMLElement {
  //#region Lifecycle

  static readonly tag = "gt-editor";

  static register(): void {
    if (customElements.get(this.tag)) return;
    customElements.define(this.tag, this);
  }

  connectedCallback(): void {
    this.#onConnect();
  }

  //#endregion

  //#region Instance

  #managerFlatPromise = flatPromise<EditorManager>();

  async #onConnect() {
    const initialState = this.#getInitialState();
    EditorManager.create({ initialState, el: this }).then((manager) =>
      this.#managerFlatPromise.resolve(manager),
    );
  }

  //#endregion

  //#region Manager

  get managerPromise(): Promise<EditorManager> {
    return this.#managerFlatPromise.promise;
  }

  //#endregion

  //#region Data

  #getInitialState(): EditorManager.State {
    const initialStateStr = this.dataset.initialState;
    if (!initialStateStr)
      throw new Error("EditorWebComponent: missing data-initial-state attribute");

    let initialStateRaw;
    try {
      initialStateRaw = JSON.parse(initialStateStr);
    } catch (err) {
      throw new Error(`EditorWebComponent: invalid JSON in data-initial-state attribute: ${err}`);
    }

    return EditorManager.State.parse(initialStateRaw);
  }

  //#endregion
}
