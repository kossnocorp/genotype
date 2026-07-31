import type { Address, Addresses, AddressId, User } from "genotype-test-map-key-alias-types";
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
