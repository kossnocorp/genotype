import cloudflare from "@astrojs/cloudflare";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";
import genotypeGrammar from "genotype-tm-grammar" with { type: "json" };

export default defineConfig({
  integrations: [
    starlight({
      title: "Genotype",
      pagefind: false,
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/kossnocorp/genotype",
        },
      ],
      sidebar: [
        {
          label: "Hello, World!",
          autogenerate: { directory: "getting-started" },
        },
        {
          label: "Language",
          autogenerate: { directory: "language" },
        },
        {
          label: "Targets",
          autogenerate: { directory: "targets" },
        },
        {
          label: "Toolchain",
          autogenerate: { directory: "toolchain" },
        },
      ],
      customCss: ["./src/styles/global.css"],
      expressiveCode: {
        shiki: {
          langs: [genotypeGrammar],
        },
      },
    }),
  ],

  vite: {
    plugins: [tailwindcss() as any],
  },

  adapter: cloudflare(),
});
