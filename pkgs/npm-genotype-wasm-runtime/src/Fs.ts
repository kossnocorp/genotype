import { minimatch } from "minimatch";
import * as path from "pathe";
import { z } from "zod";

export namespace GtwmFs {
  export interface Props {
    files: GtwmFs.Files;
    onFileChange?: GtwmFs.OnFileChange;
  }

  export type Files = z.infer<typeof GtwmFs.Files>;

  export type OnFileChange = (filePath: string, content: string | null) => unknown;
}

export class GtwmFs {
  static Files = z.record(z.string(), z.string());

  #files = new Map<string, string>();
  #onFileChange: GtwmFs.OnFileChange | undefined;

  constructor(props: GtwmFs.Props) {
    const { files, onFileChange } = props;

    for (const [filePath, source] of Object.entries(files)) {
      this.writeFile(filePath, source);
    }

    this.#onFileChange = onFileChange;
  }

  readFile(filePath: string): string | null {
    return this.#files.get(filePath) ?? null;
  }

  writeFile(filePath: string, source: string): void {
    this.#files.set(filePath, source);
    this.#onFileChange?.(filePath, source);
  }

  removeFile(filePath: string): void {
    this.#files.delete(filePath);
    this.#onFileChange?.(filePath, null);
  }

  isFile(filePath: string): boolean {
    return this.#files.has(filePath);
  }

  glob(pattern: string): string[] {
    return this.listFiles().filter((filePath) => minimatch(filePath, pattern));
  }

  findFile(fileName: string, basePath = "."): string | null {
    let currentPath = basePath;

    while (true) {
      const candidate = path.join(currentPath, fileName);

      if (this.isFile(candidate)) return candidate;

      if (currentPath === "") break;

      currentPath = path.dirname(currentPath);
    }

    return null;
  }

  listFiles(): string[] {
    return [...this.#files.keys()].sort();
  }

  snapshot(): GtwmFs.Files {
    return Object.fromEntries(this.#files.entries());
  }
}
