import cp from "node:child_process";
import crypto from "node:crypto";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { describe, expect, it } from "vitest";
import type { Nodes } from "mdast";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkGfm from "remark-gfm";
import { Mdx } from "./Mdx.ts";

const plainParser = unified().use(remarkParse).use(remarkGfm);
function nodes(node: Nodes): Nodes[] {
  return [node, ...("children" in node ? node.children.flatMap(nodes) : [])];
}

describe("Mdx", () => {
  describe("render", () => {
    it("renders MDX doc", () => {
      const source = `---
title: >-
  A title with a colon: and quotes
---

import {
  Tabs,
  TabItem,
} from "@astrojs/starlight/components";

<Tabs>
  <TabItem
    label='Rust & Python'
  >
    ~~~rust
    // <Aside> is code, not a component
    type Values = Vec<String>;


    // Preserve blank lines above.
    ~~~

  </TabItem>
</Tabs>

Before{/* hidden comment */}after.

| Name | Value |
| --- | --- |
| A | B |

[Configuration][config]

[config]: /docs/toolchain/configuration/#generation
`;
      const result = Mdx.render(source, {});
      expect(result).toMatchInlineSnapshot(`
      "# A title with a colon: and quotes

      **Rust & Python**

      \`\`\`rust
      // <Aside> is code, not a component
      type Values = Vec<String>;


      // Preserve blank lines above.
      \`\`\`

      Beforeafter.

      | Name | Value |
      | ---- | ----- |
      | A    | B     |

      [Configuration][config]

      [config]: https://genotype-lang.org/docs/toolchain/configuration/#generation
      "
    `);
    });

    it("reports unsupported expressions", () => {
      for (const body of [
        "{getExample()}",
        '<Unknown title="Example" />',
        "<TabItem label={getLabel()}>Example</TabItem>",
      ]) {
        expect(() => Mdx.render(`---\ntitle: Test\n---\n\n${body}`, {})).toThrow(
          /Unsupported|Expected a literal/,
        );
      }
    });
  });
});
