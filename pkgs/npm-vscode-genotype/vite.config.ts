import { defineConfig } from "vite";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

export default defineConfig(({ mode }) => ({
  plugins: [
    {
      name: "genotype-formatter-wasm",
      generateBundle() {
        this.emitFile({
          type: "asset",
          fileName: "genotype_formatters_wasm_bg.wasm",
          source: readFileSync(
            require.resolve("@genotype-lang/formatters-wasm/genotype_formatters_wasm_bg.wasm"),
          ),
        });
      },
    },
  ],
  ssr: {
    noExternal: true,
    external: ["vscode"],
  },
  build: {
    outDir: mode === "development" ? "dist/dev" : "dist/production",
    ssr: "./src/extension.ts",
    target: "node22",
    sourcemap: true,
    rolldownOptions: {
      external: ["vscode"],
      output: { format: "cjs", entryFileNames: "extension.js" },
    },
  },
}));
