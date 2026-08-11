import type {
  Address,
  Addresses,
  AddressId,
  BooleanAliasMap,
  BooleanMap,
  BrandedBooleanMap,
  DirectBooleanMap,
  User,
} from "genotype-test-map-key-alias-types";
import assert from "node:assert/strict";

const addressId = "home" as AddressId;
const address: Address = { street: "Main Street" };
const addresses: Addresses = { [addressId]: address };
const user: User = { addresses };

assert.deepEqual(user, {
  addresses: {
    home: { street: "Main Street" },
  },
});

const invalidAddresses: Addresses = {
  // @ts-expect-error Record keys must use the branded alias.
  home: address,
};
void invalidAddresses;

const booleanMaps: [
  DirectBooleanMap,
  BooleanMap,
  BooleanAliasMap,
  BrandedBooleanMap,
] = [
  { true: "yes", false: "no" },
  { true: "yes", false: "no" },
  { true: "yes", false: "no" },
  { true: "yes", false: "no" },
];
assert.equal(booleanMaps[0].true, "yes");

const invalidBooleanMap: DirectBooleanMap = {
  // @ts-expect-error Boolean record keys only allow "true" and "false".
  other: "no",
};
void invalidBooleanMap;
