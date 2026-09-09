import { Schema } from "effect";
import { LabeledNode, BigNumber, NumericMap, InlineAttachment, InlineExtended, Id, Payload, Summary } from "genotype-test-ts-effect-types";
import assert from "node:assert/strict";

const idInput = "evt_123" as Id;
const parsedId = Schema.decodeUnknownSync(Id)(idInput);
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
const parsedPayload = Schema.decodeUnknownSync(Payload)(payloadInput);
assert.deepEqual(payloadInput, parsedPayload);

const summaryInput: Summary = {
  ...payloadInput,
  kind: "summary",
  note: null,
};
const parsedSummary = Schema.decodeUnknownSync(Summary)(summaryInput);
assert.deepEqual(summaryInput, parsedSummary);

assert.equal(Schema.is(Payload)({ ...payloadInput, count: "3" }), false);
assert.equal(Schema.is(Payload)({ ...payloadInput, kind: "other" }), false);
assert.equal(Schema.is(Payload)({ ...payloadInput, coordinates: [1] }), false);
assert.equal(Schema.is(Payload)({ ...payloadInput, attachment: { name: 42 } }), false);
assert.equal(Schema.is(Payload)({ ...payloadInput, attachment: undefined }), true);

assert.equal(Schema.decodeUnknownSync(BigNumber)(123n), 123n);
assert.equal(Schema.is(BigNumber)(123), false);
assert.deepEqual(Schema.decodeUnknownSync(NumericMap)({ 1: "one" }), { 1: "one" });
assert.equal(Schema.is(NumericMap)({ 1: 42 }), false);
assert.deepEqual(Schema.decodeUnknownSync(InlineAttachment)({ filename: "test.txt" }), { filename: "test.txt" });
assert.equal(Schema.is(InlineExtended)([{ name: "test", size: 1, label: "label" }]), true);
assert.equal(Schema.is(InlineExtended)([{ name: "test", size: "bad", label: "label" }]), false);

const linkedInput: LabeledNode = {
  value: "first",
  label: "root",
  next: { value: "second" },
};
assert.deepEqual(Schema.decodeUnknownSync(LabeledNode)(linkedInput), linkedInput);
assert.equal(Schema.is(LabeledNode)({ ...linkedInput, next: { value: 42 } }), false);
