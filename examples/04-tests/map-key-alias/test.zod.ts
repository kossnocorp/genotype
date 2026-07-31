import { AddressId, Addresses, User } from "genotype-test-map-key-alias-zod-types";
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
