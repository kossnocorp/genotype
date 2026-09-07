import type * as mdast from "mdast";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkMdx from "remark-mdx";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkStringify from "remark-stringify";
import { parse as parseYaml } from "yaml";

export abstract class Mdx {
  static mdxParser = unified()
    .use(remarkParse)
    .use(remarkFrontmatter)
    .use(remarkGfm)
    .use(remarkMdx);

  static mdWriter = unified().use(remarkStringify, { bullet: "-", fences: true }).use(remarkGfm);

  static parseMdx(source: string): Mdx.RootNode {
    return Mdx.mdxParser.parse(source);
  }

  static parseFrontmatter(source: string): Record<string, unknown> | null {
    const fmNode = this.parseMdx(source).children.find((node) => node.type === "yaml");

    if (!fmNode) return null;

    const value: unknown = parseYaml(fmNode.value);
    if (!value || typeof value !== "object" || Array.isArray(value))
      throw new Error("Invalid YAML frontmatter");
    return value as Record<string, unknown>;
  }

  static render(source: string, pagesMap: Mdx.PagesMap): string {
    const tree = this.parseMdx(source);
    const fm = this.parseFrontmatter(source);

    const title = typeof fm?.title === "string" && fm.title.trim();
    if (!title) throw new Error("Missing or empty title in frontmatter");

    tree.children = [
      {
        type: "heading",
        depth: 1,
        children: [{ type: "text", value: title }],
      },
      ...(tree.children.flatMap((child) =>
        Mdx.#convertNode(child, pagesMap),
      ) as Mdx.RootChildNode[]),
    ];
    return this.mdWriter.stringify(tree);
  }

  static #renderJsxAttr(node: Mdx.JsxNode, name: string, fallback?: string): string {
    const attr = node.attributes.find(
      (attr) => attr.type === "mdxJsxAttribute" && attr.name === name,
    );

    if (!attr && fallback !== undefined) return fallback;

    if (attr?.type !== "mdxJsxAttribute" || typeof attr.value !== "string") {
      throw new Error(`Expected a literal ${name} on ${node.name}`);
    }

    return attr.value;
  }

  static #renderJsxLabel(node: Mdx.JsxNode): string {
    switch (node.name) {
      case "TabItem":
        return this.#renderJsxAttr(node, "label");

      case "Aside": {
        const type = this.#renderJsxAttr(node, "type", "note");
        return `${type.charAt(0).toUpperCase()}${type.slice(1)}:`;
      }

      default:
        throw new Error(`Unsupported MDX component: ${node.name}`);
    }
  }

  static #convertNode(node: Mdx.Node, pagesMap: Mdx.PagesMap): Mdx.Node[] {
    switch (node.type) {
      case "yaml":
      case "mdxjsEsm":
        // Ignore YAML frontmatter and ESM imports
        return [];

      case "mdxFlowExpression":
      case "mdxTextExpression":
        // Ignore comments e.g. `{/* comment */}`
        if (!node.data?.estree?.body.length) return [];

        // Never evaluate MDX.
        throw new Error("Unsupported MDX expression in skill reference");

      case "mdxJsxFlowElement":
      case "mdxJsxTextElement": {
        // Render JSX elements.

        const children = node.children.flatMap((child) => this.#convertNode(child, pagesMap));

        if (node.name === "Tabs" || node.name === null) return children;

        const label = this.#renderJsxLabel(node);
        return [
          {
            type: "paragraph",
            children: [
              {
                type: "strong",
                children: [{ type: "text", value: label }],
              },
            ],
          },
          ...children,
        ];
      }

      case "link":
      case "definition":
      case "image":
        if (node.url.startsWith("/") && !node.url.startsWith("//")) {
          const url = new URL(node.url, "https://genotype-lang.org");
          let key = url.pathname.startsWith("/docs/") ? url.pathname.slice(6) : url.pathname;
          if (key.endsWith("/")) key = key.slice(0, -1);
          const target = pagesMap[key] ?? pagesMap[`${key}/index`];
          node.url = target ? `${target}${url.search}${url.hash}` : url.href;
        }
    }

    if ("children" in node) {
      node.children = node.children.flatMap((child) =>
        this.#convertNode(child, pagesMap),
      ) as typeof node.children;
    }
    return [node];
  }
}

export namespace Mdx {
  export type PagesMap = Record<string, string>;

  export type Node = mdast.Nodes;

  export type RootNode = mdast.Root;

  export type RootChildNode = mdast.RootContent;

  export type JsxNode = JsxFlowNode | JsxTextNode;

  export type JsxFlowNode = Node & { type: "mdxJsxFlowElement" };

  export type JsxTextNode = Node & { type: "mdxJsxTextElement" };
}
