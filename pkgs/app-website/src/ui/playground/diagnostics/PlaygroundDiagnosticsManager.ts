import type * as Gt from "@genotype-lang/types";
import { always } from "alwaysly";
import { Wc } from "../../utils/Wc";
import type { PlaygroundDiagnosticsWebComponent } from "./PlaygroundDiagnosticsWebComponent";

export namespace PlaygroundDiagnosticsManager {
  export interface Props {
    wc: PlaygroundDiagnosticsWebComponent;
  }
}

export class PlaygroundDiagnosticsManager {
  #wc: PlaygroundDiagnosticsWebComponent;

  constructor(props: PlaygroundDiagnosticsManager.Props) {
    this.#wc = props.wc;
  }

  clear(): void {
    this.#wc.clear();
  }

  reportDiagnostic(diagnostic: Gt.GtDiagnostic): void {
    const shouldScroll = this.#isScrolledToBottom();
    const templateEl = this.#wc.locate('template[data-id="diagnostic"]', HTMLTemplateElement);
    const fragment = templateEl.content.cloneNode(true);
    always(fragment instanceof DocumentFragment);

    const diagnosticEl = Wc.locate(fragment, "[data-diagnostic]");
    diagnosticEl.dataset.kind = diagnostic.kind;

    const labelEl = Wc.locate(fragment, "[data-diagnostic-label]");
    labelEl.textContent = this.#label(diagnostic.kind);

    const titleEl = Wc.locate(fragment, "[data-diagnostic-title]");
    titleEl.textContent = `${diagnostic.content.title}.`;

    const contentEl = Wc.locate(fragment, "[data-diagnostic-content]");
    const bodyTemplateEl = this.#wc.locate(
      'template[data-id="diagnostic-body"]',
      HTMLTemplateElement,
    );
    const bodies =
      "report" in diagnostic.content
        ? [diagnostic.content.report]
        : typeof diagnostic.content.body === "string"
          ? [diagnostic.content.body]
          : diagnostic.content.body;

    for (const body of bodies ?? []) {
      const bodyFragment = bodyTemplateEl.content.cloneNode(true);
      always(bodyFragment instanceof DocumentFragment);
      const bodyEl = Wc.locate(bodyFragment, "[data-diagnostic-body]");
      bodyEl.textContent = body.trim();
      contentEl.appendChild(bodyFragment);
    }

    contentEl.hidden = contentEl.childElementCount === 0;
    this.#wc.appendChild(fragment);

    if (shouldScroll) this.#wc.scrollTop = this.#wc.scrollHeight;
  }

  #isScrolledToBottom(): boolean {
    return this.#wc.scrollHeight - this.#wc.scrollTop - this.#wc.clientHeight <= 1;
  }

  #label(kind: Gt.GtDiagnosticKind): string {
    return kind[0].toUpperCase() + kind.slice(1);
  }
}
