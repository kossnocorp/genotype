import { shikiToMonaco } from "@shikijs/monaco";
import * as monaco from "monaco-editor-core/esm/vs/editor/editor.api";
import { createHighlighterCore } from "shiki/core";
import { createJavaScriptRegexEngine } from "shiki/engine/javascript";
import EditorWorker from "./EditorWorker?worker";
import { z } from "zod";

export namespace EditorManager {
  export type State = z.infer<typeof EditorManager.State>;

  export type OnCodeChange = (state: State & {}) => void;

  export interface Props {
    initialState: State;
    el: HTMLElement;
  }
}

export class EditorManager {
  //#region Schema

  static State = z
    .object({
      filePath: z.string(),
      code: z.union([z.string(), z.undefined(), z.null()]),
    })
    .nullable();

  //#endregion

  //#region Instance

  static async create(props: EditorManager.Props): Promise<EditorManager> {
    await Promise.all([
      EditorManager.initMonaco(),
      document.fonts.load('14px "JetBrains Mono Variable"'),
    ]);
    return new EditorManager(props);
  }

  #state: EditorManager.State = null;
  #model: monaco.editor.ITextModel = EditorManager.#createModel(null);
  #editor: monaco.editor.IStandaloneCodeEditor;
  #onCodeChangeCallbacks = new Set<EditorManager.OnCodeChange>();
  #ignoreModelChanges = false;

  constructor(props: EditorManager.Props) {
    const { el, initialState } = props;

    this.setState(initialState);

    this.#editor = monaco.editor.create(el, {
      model: this.#model,
      automaticLayout: true,
      minimap: { enabled: false },
      lineNumbersMinChars: 3,
      roundedSelection: false,
      smoothScrolling: true,
      scrollBeyondLastLine: false,
      fontSize: 14,
      lineHeight: 24,
      fontFamily: "var(--font-mono)",
      padding: { top: 10, bottom: 10 },
    });

    this.#editor.onDidChangeModelContent(() => {
      if (this.#ignoreModelChanges || !this.#state) return;

      const state = { filePath: this.#state.filePath, code: this.#model.getValue() };
      this.#state = state;
      this.#onCodeChangeCallbacks.forEach((callback) => callback(state));
    });
  }

  //#endregion

  //#region Events

  onCodeChange(callback: EditorManager.OnCodeChange): () => void {
    this.#onCodeChangeCallbacks.add(callback);
    return () => this.#onCodeChangeCallbacks.delete(callback);
  }

  //#endregion

  //#region State

  trySetState(state: EditorManager.State & {}): void {
    if (state.filePath !== this.#state?.filePath) return;
    this.setState(state);
  }

  setState(state: EditorManager.State): void {
    const curState = this.#state;
    if (EditorManager.#areStatesEqual(curState, state)) return;

    this.#state = state;

    if (curState?.filePath !== state?.filePath) return this.#updateModel(state);

    this.#setModelValue(state?.code);
  }

  static #areStatesEqual(state1: EditorManager.State, state2: EditorManager.State): boolean {
    // If one of the states is null, check if the other is null.
    if (!state1 || !state2) return state1 === state2;
    // Check internals
    return state1.filePath === state2.filePath && state1.code === state2.code;
  }

  //#endregion

  //#region Model

  #updateModel(state: EditorManager.State): void {
    const model = EditorManager.#createModel(state);
    this.#model = model;
    this.#editor.setModel(model);
  }

  #setModelValue(code: string | undefined | null) {
    this.#ignoreModelChanges = true;
    try {
      this.#model.setValue(code ?? "");
    } finally {
      this.#ignoreModelChanges = false;
    }
  }

  static #createModel(state: EditorManager.State) {
    return monaco.editor.createModel(state?.code ?? "", this.#detectLang(state?.filePath));
  }

  //#endregion

  //#region Interop

  static #monacoReadyPromise: Promise<void> | null = null;

  static initMonaco(): Promise<void> {
    if (!this.#monacoReadyPromise) this.#monacoReadyPromise = this.#initMonaco();
    return this.#monacoReadyPromise;
  }

  static async #initMonaco() {
    // TODO: Simplify when the issue is resolved: https://github.com/microsoft/monaco-editor/issues/2605

    this.#initMonacoEnvironment();

    const langs = this.#langs();

    Object.keys(langs).forEach((id) => monaco.languages.register({ id }));

    const highlighter = await createHighlighterCore({
      langs: Object.values(langs),
      themes: [
        import("../../vendor/sophia-dark.json").then((theme) => {
          const {
            name: displayName,
            type,
            semanticHighlighting,
            colors,
            tokenColors: settings,
          } = theme;
          return {
            name: "sophia-dark",
            displayName,
            type: type as "dark" | "light",
            semanticHighlighting,
            colors,
            settings,
          };
        }),

        import("shiki/themes/github-dark.mjs"),
      ],
      engine: createJavaScriptRegexEngine(),
    });

    shikiToMonaco(highlighter, monaco);

    monaco.editor.setTheme("sophia-dark");
  }

  static #initMonacoEnvironment(): void {
    // TODO: Simplify when the issue is resolved: https://github.com/microsoft/monaco-editor/issues/2605

    type GlobalThisWithMonacoEnvironment = typeof globalThis & {
      MonacoEnvironment?: { getWorker: () => Worker };
    };

    (globalThis as GlobalThisWithMonacoEnvironment).MonacoEnvironment = {
      getWorker: () => new EditorWorker(),
    };
  }

  //#endregion

  //#region Langs

  static #langs() {
    return {
      toml: import("@shikijs/langs/toml"),
      json: import("@shikijs/langs/json"),
      typescript: import("@shikijs/langs/typescript"),
      python: import("@shikijs/langs/python"),
      rust: import("@shikijs/langs/rust"),
      genotype: import("@genotype-lang/grammar-tm"),
    };
  }

  static #detectLang(filePath: string | undefined): string {
    if (typeof filePath === "string") {
      if (filePath.endsWith(".toml")) return "toml";
      if (filePath.endsWith(".json")) return "json";
      if (filePath.endsWith(".ts")) return "typescript";
      if (filePath.endsWith(".py")) return "python";
      if (filePath.endsWith(".rs")) return "rust";
      if (filePath.endsWith(".type")) return "genotype";
    }
    return "plaintext";
  }

  //#endregion
}
