import { JsonAny } from "genotype-test-cyclic-refs-ts-zod";
import assert from "node:assert/strict";

const input = {
  kind: "object",
  name: "root",
  properties: [
    {
      kind: "property",
      name: "children",
      descriptor: {
        kind: "array",
        descriptor: {
          kind: "object",
          properties: [
            {
              kind: "property",
              name: "name",
              descriptor: {
                kind: "string",
              },
              required: true,
            },
          ],
        },
      },
      required: true,
    },
  ],
};
const parsed = JsonAny.parse(input);
assert.deepEqual(parsed, input);
