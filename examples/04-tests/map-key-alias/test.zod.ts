import {
  AddressId,
  Addresses,
  BooleanAliasMap,
  BooleanMap,
  BrandedBooleanMap,
  DirectBooleanMap,
  User,
} from "genotype-test-map-key-alias-zod-types";
import assert from "node:assert/strict";

const addressId = AddressId.parse("home");

assert.deepEqual(
  Addresses.parse({
    [addressId]: { street: "Main Street" },
  }),
  {
    home: { street: "Main Street" },
  },
);

assert.deepEqual(
  User.parse({
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

assert.equal(Addresses.safeParse({ home: { street: 42 } }).success, false);

for (const schema of [
  DirectBooleanMap,
  BooleanMap,
  BooleanAliasMap,
  BrandedBooleanMap,
]) {
  assert.deepEqual(schema.parse({ true: "yes", false: "no" }), {
    true: "yes",
    false: "no",
  });
  assert.equal(
    schema.safeParse({ true: "yes", false: "no", other: "no" }).success,
    false,
  );
}
