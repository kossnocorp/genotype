import assert from "node:assert/strict";

import type {
  LiteralBag,
  RuntimeResponse,
  RuntimeResponseFailure,
  RuntimeResponseSuccess,
  RuntimeStatus,
} from "genotype-test-literals-ts";

const success: RuntimeResponseSuccess = {
  status: "success",
  value: "hello",
};

const failure: RuntimeResponseFailure = {
  status: "failure",
  error: "boom",
};

const responses: RuntimeResponse[] = [success, failure];

const statusSuccess: RuntimeStatus = "success";
const statusFailure: RuntimeStatus = "failure";

const bag: LiteralBag = {
  kind: "demo",
  enabled: true,
  code: 200,
  empty: null,
};

assert.equal(responses[0]?.status, statusSuccess);
assert.equal(responses[1]?.status, statusFailure);
assert.equal(bag.kind, "demo");
