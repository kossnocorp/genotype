import cloudflare from "@astrojs/cloudflare";
import starlight from "@astrojs/starlight";
import genotypeGrammar from "@genotype-lang/grammar-tm" with { type: "json" };
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";

const localHost = process.env.LOCAL_HOST;

export default defineConfig({
  integrations: [
    starlight({
      title: "Genotype",
      head: [
        {
          tag: "link",
          attrs: {
            rel: "icon",
            href: "/favicon.ico",
            sizes: "32x32",
          },
        },
        {
          tag: "link",
          attrs: {
            rel: "icon",
            href: "/favicon.svg",
            type: "image/svg+xml",
          },
        },
        {
          tag: "link",
          attrs: {
            rel: "apple-touch-icon",
            href: "/apple-touch-icon.png",
          },
        },
        {
          tag: "link",
          attrs: {
            rel: "manifest",
            href: "/manifest.webmanifest",
          },
        },
      ],
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
          items: [{ autogenerate: { directory: "docs/getting-started" } }],
        },
        {
          label: "Language",
          items: [{ autogenerate: { directory: "docs/language" } }],
        },
        {
          label: "Targets",
          items: [
            {
              label: "TypeScript",
              items: [{ autogenerate: { directory: "docs/targets/typescript" } }],
            },
            {
              label: "Rust",
              items: [{ autogenerate: { directory: "docs/targets/rust" } }],
            },
            {
              label: "Python",
              items: [{ autogenerate: { directory: "docs/targets/python" } }],
            },
          ],
        },
        {
          label: "Toolchain",
          items: [{ autogenerate: { directory: "docs/toolchain" } }],
        },
      ],
      customCss: [
        "./src/styles/global.css",
        "@fontsource-variable/mona-sans",
        "@fontsource-variable/jetbrains-mono",
        "@fontsource-variable/hubot-sans",
      ],
      components: {
        // Layout
        Header: "./src/ui/layout/LayoutTopbar.astro",
        PageFrame: "./src/ui/layout/LayoutFrame.astro",
        TwoColumnContent: "./src/ui/layout/LayoutContent.astro",
        ContentPanel: "./src/ui/layout/LayoutContentPanel.astro",
        // General overrides
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
    server: {
      allowedHosts: localHost ? [localHost] : [],
    },
  },

  adapter: cloudflare(),
});
