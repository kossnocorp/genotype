import { flatPromise } from "@js-fns/promise";
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
    this.locateAll("[data-diagnostics-toggle]").forEach((toggle) => {
      toggle.onclick = () => this.#setExpanded(this.panel.hidden !== false);
    });
    this.locate("[data-diagnostics-resize]").onclick = () => {
      this.#setEnlarged(this.dataset.enlarged !== "true");
    };
    this.#managerFlatPromise.resolve(new PlaygroundDiagnosticsManager({ wc: this }));
  }

  clear(): void {
    this.locateAll("[data-diagnostic]").forEach((diagnosticEl) => diagnosticEl.remove());
    this.messages.scrollTop = 0;
    this.updateStatus();
  }

  get panel(): HTMLElement {
    return this.locate("[data-diagnostics-panel]");
  }

  get messages(): HTMLElement {
    return this.locate("[data-diagnostics-messages]");
  }

  updateStatus(): void {
    const errors = this.locateAll('[data-diagnostic][data-kind="error"]').length;
    const warnings = this.locateAll('[data-diagnostic][data-kind="warning"]').length;
    const infos = this.messages.childElementCount - errors - warnings;
    const counts = { info: infos, warning: warnings, error: errors };
    const status = errors ? "error" : warnings ? "warning" : "info";
    const count = errors || warnings || this.messages.childElementCount;
    const noun = errors ? "error" : warnings ? "warning" : "message";
    this.dataset.status = status;
    this.locate("[data-diagnostics-count]").textContent = String(count);
    this.locateAll("[data-status-group]").forEach((group) => {
      const kind = group.dataset.statusGroup as keyof typeof counts;
      group.hidden = counts[kind] === 0;
      Wc.locate(group, "[data-status-count]").textContent = String(counts[kind]);
    });
    this.locateAll("[data-status-icon]").forEach((icon) => {
      icon.hidden = icon.dataset.statusIcon !== status;
    });
    const action = this.panel.hidden ? "Show" : "Collapse";
    const label = `${action} diagnostics: ${count} ${noun}${count === 1 ? "" : "s"}`;
    this.locateAll("[data-diagnostics-toggle]").forEach((toggle) => {
      toggle.setAttribute("aria-label", label);
      toggle.title = label;
    });
  }

  expand(): void {
    if (this.panel.hidden !== false) {
      this.#setExpanded(true);
    } else {
      this.updateStatus();
      this.#scrollToFirstError();
    }
  }

  #setEnlarged(enlarged: boolean): void {
    this.dataset.enlarged = String(enlarged);
    this.locate('[data-resize-icon="expand"]').hidden = enlarged;
    this.locate('[data-resize-icon="collapse"]').hidden = !enlarged;
    const resize = this.locate("[data-diagnostics-resize]");
    const label = enlarged ? "Restore diagnostics height" : "Expand diagnostics to 60%";
    resize.setAttribute("aria-pressed", String(enlarged));
    resize.setAttribute("aria-label", label);
    resize.title = label;
  }

  #setExpanded(expanded: boolean): void {
    const opening = expanded && this.panel.hidden !== false;
    const toggles = this.locateAll("[data-diagnostics-toggle]");
    const hadFocus = toggles.some((toggle) => toggle === document.activeElement);
    if (!expanded) this.#setEnlarged(false);
    this.panel.hidden = !expanded;
    this.locate(".badge").hidden = expanded;
    toggles.forEach((toggle) => {
      toggle.setAttribute("aria-expanded", String(expanded));
    });
    this.updateStatus();
    if (hadFocus) {
      const target = expanded
        ? Wc.locate(this.panel, "[data-diagnostics-toggle]")
        : this.locate(".badge");
      target.focus();
    }
    if (opening) this.#scrollToFirstError();
  }

  #scrollToFirstError(): void {
    const messages = this.messages;
    const firstError = messages.querySelector<HTMLElement>('[data-diagnostic][data-kind="error"]');
    if (firstError) {
      messages.scrollTop +=
        firstError.getBoundingClientRect().top - messages.getBoundingClientRect().top;
    }
  }

  get managerPromise(): Promise<PlaygroundDiagnosticsManager> {
    return this.#managerFlatPromise.promise;
  }
}
