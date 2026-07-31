import { GtwmClient, GtwmFs } from "@genotype-lang/wasm-runtime";
import type * as Gt from "@genotype-lang/types";
import { z } from "zod";
import type { PlaygroundWebComponent } from "./PlaygroundWebComponent";
import type { EditorWebComponent } from "../editor/EditorWebComponent";
import type { EditorManager } from "../editor/EditorManager";
import { resettablePromise } from "@js-fns/promise";
import { always } from "alwaysly";
import type { PlaygroundDiagnosticsWebComponent } from "./diagnostics/PlaygroundDiagnosticsWebComponent";

export namespace PlaygroundManager {
  export type Lang = z.infer<typeof PlaygroundManager.Lang>;

  export type State = z.infer<typeof PlaygroundManager.State>;

  export type InitialState = z.infer<typeof PlaygroundManager.InitialState>;

  export interface Props {
    wc: PlaygroundWebComponent;
    initialState: InitialState;
    sourceEditorWc: EditorWebComponent;
    distEditorWc: EditorWebComponent;
    diagnosticsWc: PlaygroundDiagnosticsWebComponent;
  }
}

export class PlaygroundManager {
  //#region Schema

  static Lang = z.enum(["ts", "rs", "py"]);

  static State = z.object({
    lang: this.Lang,
    filePath: z.string().nullable(),
  });

  static InitialState = this.State.extend({
    files: GtwmFs.Files,
  });

  //#endregion

  #state: PlaygroundManager.State;

  #wc: PlaygroundWebComponent;
  #sourceEditorWc: EditorWebComponent;
  #distEditorWc: EditorWebComponent;
  #diagnosticsWc: PlaygroundDiagnosticsWebComponent;

  #fs: GtwmFs;

  #compiledMetaPromise = resettablePromise<Gt.GtcMetaCompiled>();
  #compileTimer: ReturnType<typeof setTimeout> | undefined;

  constructor(props: PlaygroundManager.Props) {
    const {
      wc: el,
      initialState: { files, ...state },
      sourceEditorWc,
      distEditorWc,
      diagnosticsWc,
    } = props;

    this.#state = state;
    this.#wc = el;
    this.#distEditorWc = distEditorWc;
    this.#sourceEditorWc = sourceEditorWc;
    this.#diagnosticsWc = diagnosticsWc;

    this.#fs = new GtwmFs({ files, onFileChange: this.#onFsCodeChange.bind(this) });

    this.#connectSourceEditor();
    this.compile();
  }

  //#region Compilation

  async compile(): Promise<void> {
    this.#compiledMetaPromise.reset();

    try {
      const diagnosticsManager = await this.#diagnosticsWc.managerPromise;
      diagnosticsManager.clear();

      // TODO: Add project reload support and reuse the same GtwmClient instance.
      const gt = new GtwmClient({ fs: this.#fs, onDiagnostic: this.#onDiagnostic.bind(this) });

      try {
        const _loadedProjectMeta = await gt.loadInProject();

        const loadedModulesMeta = await gt.loadInModules();

        if (!this.#state.filePath) {
          const modules = [...loadedModulesMeta.modules];
          modules.sort();
          const firstModuleFilePath = modules[0];
          if (firstModuleFilePath) this.#state.filePath = firstModuleFilePath;
        }

        this.#wc.updateFileTabs({
          srcPath: loadedModulesMeta.paths.src,
          modulePaths: loadedModulesMeta.modules,
          sourceFilePath: this.#state.filePath,
        });

        const compiledMeta = await gt.compile();

        this.#compiledMetaPromise.resolve(compiledMeta);

        const filePath = this.#state.filePath ?? compiledMeta.modules[0].src;
        if (!filePath) return;

        await this.openFile(filePath);
      } finally {
        gt.dispose();
      }
    } catch (error) {
      this.#compiledMetaPromise.reject(error);
    }
  }

  async #currentModule(): Promise<Gt.GtcMetaCompiledModule> {
    const compiledMeta = await this.#compiledMetaPromise.promise;

    const module = compiledMeta.modules.find((module) => module.src === this.#state.filePath);
    always(module);
    return module;
  }

  //#endregion

  //#region Files

  async openFile(filePath: string): Promise<void> {
    this.#state.filePath = filePath;

    const module = await this.#currentModule();

    await Promise.all([
      this.#setEditorStateWithFilePath(this.#sourceEditorWc, module.src),
      this.#setEditorStateWithFilePath(this.#distEditorWc, module[this.#state.lang]),
    ]);
  }

  //#endregion

  //#region Langs

  async setLang(lang: PlaygroundManager.Lang): Promise<void> {
    this.#state.lang = lang;

    const module = await this.#currentModule();

    return this.#setEditorStateWithFilePath(this.#distEditorWc, module[this.#state.lang]);
  }

  //#endregion

  //#region Code

  async #onFsCodeChange(filePath: string, code: string | null): Promise<void> {
    const state: EditorManager.State = { filePath, code };

    await Promise.all([
      this.#trySetEditorState(this.#sourceEditorWc, state),
      this.#trySetEditorState(this.#distEditorWc, state),
    ]);
  }

  #onEditorCodeChange(filePath: string, code: string | undefined | null): void {
    this.#fs.writeFile(filePath, code ?? "");

    clearTimeout(this.#compileTimer);
    this.#compileTimer = setTimeout(() => this.compile(), 250);
  }

  //#endregion

  //#region Editors

  async #connectSourceEditor(): Promise<void> {
    const manager = await this.#sourceEditorWc.managerPromise;

    manager.onCodeChange(({ filePath, code }) => this.#onEditorCodeChange(filePath, code));
  }

  async #setEditorStateWithFilePath(
    wc: EditorWebComponent,
    filePath: string | undefined,
  ): Promise<void> {
    if (!filePath) return this.#setEditorState(wc, null);

    const code = this.#fs.readFile(filePath);
    return this.#setEditorState(wc, { filePath, code });
  }

  async #setEditorState(wc: EditorWebComponent, state: EditorManager.State): Promise<void> {
    const manager = await wc.managerPromise;
    manager.setState(state);
  }

  async #trySetEditorState(wc: EditorWebComponent, state: EditorManager.State & {}): Promise<void> {
    const manager = await wc.managerPromise;
    manager.trySetState(state);
  }

  //#endregion

  //#region Diagnostics

  async #onDiagnostic(diagnostic: Gt.GtDiagnostic): Promise<void> {
    const manager = await this.#diagnosticsWc.managerPromise;
    manager.reportDiagnostic(diagnostic);
  }

  //#endregion
}
