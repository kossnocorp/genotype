import { Schema } from "effect";
import { JsonAny } from "genotype-test-cyclic-refs-ts-effect";
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
const parsed = Schema.decodeUnknownSync(JsonAny)(input);
assert.deepEqual(parsed, input);
