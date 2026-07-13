import { always } from "alwaysly";
import { PlaygroundManager } from "./PlaygroundManager";
import { z } from "zod";
import { EditorWebComponent } from "../editor/EditorWebComponent";
import { Wc } from "../utils/Wc";
import { PlaygroundDiagnosticsWebComponent } from "./diagnostics/PlaygroundDiagnosticsWebComponent";

export namespace PlaygroundWebComponent {
  export interface UpdateFileTabsProps {
    srcPath: string;
    modulePaths: string[];
    sourceFilePath: string | null;
  }
}

export class PlaygroundWebComponent extends Wc {
  //#region Lifecycle

  static readonly tag = "gt-playground";

  static register(): void {
    if (customElements.get(this.tag)) return;
    customElements.define(this.tag, this);
  }

  connectedCallback(): void {
    this.#onConnect();
  }

  //#endregion

  //#region Initialization

  #manager: PlaygroundManager | undefined;

  async #onConnect() {
    const initialState = this.#getInitialState();
    // this.#sourceFilePath = initialState.sourceFilePath;

    const sourceEditorWc = this.#locateEditorWc("source");
    const distEditorWc = this.#locateEditorWc("dist");
    const diagnosticsWc = this.locate(
      PlaygroundDiagnosticsWebComponent.tag,
      PlaygroundDiagnosticsWebComponent,
    );

    this.#manager = new PlaygroundManager({
      wc: this,
      initialState,
      sourceEditorWc,
      distEditorWc,
      diagnosticsWc,
    });

    this.#connectFileTabChangeListener();
    this.#connectLangChangeListener();
  }

  #getInitialState(): PlaygroundManager.InitialState {
    const initialStateStr = this.dataset.initialState;
    if (!initialStateStr)
      throw new Error("PlaygroundWebComponent: missing data-initial-state attribute");

    let initialStateRaw;
    try {
      initialStateRaw = JSON.parse(initialStateStr);
    } catch (err) {
      throw new Error(
        `PlaygroundWebComponent: invalid JSON in data-initial-state attribute: ${err}`,
      );
    }

    return PlaygroundManager.InitialState.parse(initialStateRaw);
  }

  //#endregion

  //#region Files

  updateFileTabs(props: PlaygroundWebComponent.UpdateFileTabsProps): void {
    this.#renderFileTabs(props);
  }

  #renderFileTabs(props: PlaygroundWebComponent.UpdateFileTabsProps): void {
    const { srcPath, modulePaths, sourceFilePath } = props;

    const fileTabsEl = this.locate("[data-file-tabs]");
    const templateEl = this.locate('template[data-id="file-tab"]', HTMLTemplateElement);

    fileTabsEl.replaceChildren();

    for (const modulePath of modulePaths) {
      const fragment = templateEl.content.cloneNode(true);
      always(fragment instanceof DocumentFragment);

      const fileTabEl = Wc.locate(fragment, "[data-file-tab]");

      fileTabEl.dataset.filePath = modulePath;
      fileTabEl.classList.toggle("is-active", modulePath === sourceFilePath);

      const labelEl = Wc.locate(fragment, "[data-file-tab-label]");

      labelEl.textContent = this.#filePathWithoutSrcPrefix(modulePath, srcPath);

      fileTabsEl.appendChild(fragment);
    }
  }

  #connectFileTabChangeListener() {
    const fileTabsEl = this.locate("[data-file-tabs]");

    fileTabsEl.addEventListener("click", (event) => {
      if (!(event.target instanceof Element)) return;

      const fileTabEl = Wc.locateClosest(event.target, "[data-file-tab]");
      if (!(fileTabEl instanceof HTMLElement) || !fileTabsEl.contains(fileTabEl)) return;

      const filePath = z.string().parse(fileTabEl.dataset.filePath);
      Wc.locateEach(fileTabsEl, "[data-file-tab]", (tab) =>
        tab.classList.toggle("is-active", tab === fileTabEl),
      );

      this.#manager?.openFile(filePath);
    });
  }

  #filePathWithoutSrcPrefix(filePath: string, srcPath: string): string {
    const normalizedPrefix = srcPath.endsWith("/") ? srcPath : `${srcPath}/`;
    return filePath.startsWith(normalizedPrefix)
      ? filePath.slice(normalizedPrefix.length)
      : filePath;
  }

  //#endregion

  //#region Langs

  #connectLangChangeListener() {
    const langTabs = this.locateAll("[data-lang-tab]");

    langTabs.forEach((langTab) => {
      langTab.addEventListener("click", () => {
        const lang = PlaygroundManager.Lang.parse(langTab.dataset.lang);

        langTabs.forEach((tab) => tab.classList.toggle("is-active", tab === langTab));

        this.#manager?.setLang(lang);
      });
    });
  }

  //#endregion

  //#region Editors

  #locateEditorWc(id: string): EditorWebComponent {
    return this.locate(`${EditorWebComponent.tag}[data-id="${id}"]`, EditorWebComponent);
  }

  //#endregion
}
