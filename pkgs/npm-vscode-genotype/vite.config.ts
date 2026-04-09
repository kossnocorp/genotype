import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "dist/production",
    lib: {
      formats: ["cjs"],
      entry: "./src/extension.ts",
      fileName: "extension",
    },
    sourcemap: true,
    rolldownOptions: {
      external: ["vscode"],
    },
  },
});