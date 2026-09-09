import { Schema } from "effect";
import {
  AddressId,
  Addresses,
  BooleanAliasMap,
  BooleanMap,
  BrandedBooleanMap,
  DirectBooleanMap,
  User,
} from "genotype-test-map-key-alias-effect-types";
import assert from "node:assert/strict";

const addressId = Schema.decodeUnknownSync(AddressId)("home");

assert.deepEqual(
  Schema.decodeUnknownSync(Addresses)({
    [addressId]: { street: "Main Street" },
  }),
  {
    home: { street: "Main Street" },
  },
);

assert.deepEqual(
  Schema.decodeUnknownSync(User)({
    addresses: {
      home: { street: "Main Street" },
    },
  }),
  {
    addresses: {
      home: { street: "Main Street" },
    },
  },
);

assert.equal(Schema.is(Addresses)({ home: { street: 42 } }), false);

for (const schema of [
  DirectBooleanMap,
  BooleanMap,
  BooleanAliasMap,
  BrandedBooleanMap,
]) {
  assert.deepEqual(Schema.decodeUnknownSync(schema)({ true: "yes", false: "no" }), {
    true: "yes",
    false: "no",
  });
  assert.equal(Schema.is(schema)({ true: 42, false: "no" }), false);
  assert.throws(() =>
    Schema.decodeUnknownSync(schema, { onExcessProperty: "error" })({
      true: "yes", false: "no", other: "no",
    }),
  );
}
