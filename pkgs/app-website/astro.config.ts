import cloudflare from "@astrojs/cloudflare";
import starlight from "@astrojs/starlight";
import genotypeGrammar from "@genotype-lang/grammar-tm" with { type: "json" };
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";

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
          items: [{ autogenerate: { directory: "getting-started" } }],
        },
        {
          label: "Language",
          items: [{ autogenerate: { directory: "language" } }],
        },
        {
          label: "Targets",
          items: [{ autogenerate: { directory: "targets" } }],
        },
        {
          label: "Toolchain",
          items: [{ autogenerate: { directory: "toolchain" } }],
        },
      ],
      customCss: [
        "./src/styles/global.css",
        "@fontsource-variable/mona-sans",
        "@fontsource-variable/jetbrains-mono",
        "@fontsource-variable/hubot-sans",
      ],
      components: {
        ContentPanel: "./src/ui/starlight/ContentPanel.astro",
        PageTitle: "./src/ui/starlight/PageTitle.astro",
        MarkdownContent: "./src/ui/starlight/MarkdownContent.astro",
        Footer: "./src/ui/starlight/Footer.astro",
      },
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
