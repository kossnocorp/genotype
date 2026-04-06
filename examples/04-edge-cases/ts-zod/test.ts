import { Id, Payload, Summary } from "genotype-test-cyclic-refs-pkg";
import assert from "node:assert/strict";

const idInput = "evt_123" as Id;
const parsedId = Id.parse(idInput);
assert.equal(parsedId, idInput);

const payloadInput: Payload = {
  id: "evt_123" as Id,
  kind: "payload",
  title: "Build completed",
  count: 3,
  ratio: 0.75,
  enabled: true,
  tags: ["alpha", "release"],
  coordinates: [59.9386, 30.3141],
  metadata: {
    region: "eu",
    env: "dev",
  },
  extra: {
    retries: 1,
  },
};
const parsedPayload = Payload.parse(payloadInput);
assert.deepEqual(payloadInput, parsedPayload);

const summaryInput: Summary = {
  ...payloadInput,
  kind: "summary",
  note: null,
};
const parsedSummary = Summary.parse(summaryInput);
assert.deepEqual(summaryInput, parsedSummary);
