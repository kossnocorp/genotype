import { Schema } from "effect";
import {
  LabeledNode,
  BigNumber,
  NumericMap,
  InlineAttachment,
  InlineExtended,
  Id,
  Payload,
  Summary,
  RecursiveTuple,
  RecursiveObject,
  ExtendedRecursiveObject,
  CycleA,
  CycleB,
  ExtendedA,
  ExtendedB,
  OptionalName,
  ReversedA,
  ReversedB,
  RecursiveExtended,
  RecursiveGenericExtended,
  RecursiveConcreteExtended,
  InheritedChild,
  InheritedOverride,
} from "genotype-test-ts-effect-types";
import assert from "node:assert/strict";
import { test } from "node:test";

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

test("optional properties accept omission and explicit undefined", () => {
  const inputs: OptionalName[] = [{}, { name: undefined }, { name: "hello" }];
  for (const input of inputs) {
    assert.equal(Schema.is(OptionalName)(input), true);
    assert.deepEqual(Schema.decodeUnknownSync(OptionalName)(input), input);
    assert.deepEqual(Schema.encodeSync(OptionalName)(input), input);
  }
  assert.equal(Schema.is(OptionalName)({ name: 42 }), false);
});

test("recursive generic tuples construct lazily and validate nested values", () => {
  const schema = RecursiveTuple(Schema.String);
  const input: Schema.Schema.Type<typeof schema> = ["first", ["second", null]];
  // @ts-expect-error Recursive values must retain the generic argument type.
  const invalid: Schema.Schema.Type<typeof schema> = ["first", [42, null]];
  assert.deepEqual(Schema.decodeUnknownSync(schema)(null), null);
  assert.deepEqual(Schema.decodeUnknownSync(schema)(input), input);
  assert.deepEqual(Schema.encodeSync(schema)(input), input);
  assert.equal(Schema.is(schema)(invalid), false);
});

test("recursive generic tuples preserve transformed encoded values", () => {
  const schema = RecursiveTuple(Schema.NumberFromString);
  const encoded: Schema.Codec.Encoded<typeof schema> = ["1", ["2", null]];
  const decoded: Schema.Schema.Type<typeof schema> = [1, [2, null]];
  assert.deepEqual(Schema.decodeUnknownSync(schema)(encoded), decoded);
  assert.deepEqual(Schema.encodeSync(schema)(decoded), encoded);
});

test("recursive generic objects retain nested value types", () => {
  const schema = RecursiveObject(Schema.String);
  const input: Schema.Schema.Type<typeof schema> = { value: "first", next: { value: "second" } };
  // @ts-expect-error Recursive values must retain the generic argument type.
  const invalid: Schema.Schema.Type<typeof schema> = { value: "first", next: { value: 42 } };
  assert.deepEqual(Schema.decodeUnknownSync(schema)(input), input);
  assert.deepEqual(Schema.encodeSync(schema)(input), input);
  assert.equal(Schema.is(schema)(invalid), false);
});

test("recursive generic objects preserve transformed encoded values", () => {
  const schema = RecursiveObject(Schema.NumberFromString);
  const encoded: Schema.Codec.Encoded<typeof schema> = { value: "1", next: { value: "2" } };
  const decoded: Schema.Schema.Type<typeof schema> = { value: 1, next: { value: 2 } };
  assert.deepEqual(Schema.decodeUnknownSync(schema)(encoded), decoded);
  assert.deepEqual(Schema.encodeSync(schema)(decoded), encoded);
});

test("recursive generic structs keep fields available for extensions", () => {
  const schema = ExtendedRecursiveObject(Schema.NumberFromString);
  const encoded = { label: "root", value: "1", next: { value: "2" } };
  const decoded = { label: "root", value: 1, next: { value: 2 } };
  assert.deepEqual(Schema.decodeUnknownSync(schema)(encoded), decoded);
  assert.deepEqual(Schema.encodeSync(schema)(decoded), encoded);
  assert.equal(Schema.is(schema)({ label: "root", value: 1, next: { value: "2" } }), false);
});

test("cyclic extensions retain required inherited fields", () => {
  // Every level requires another `a`; no finite object satisfies this cycle.
  for (const schema of [CycleA, CycleB]) {
    for (const input of [{}, { a: {} }, { a: { a: {} } }, { a: 42 }]) {
      assert.equal(Schema.is(schema)(input), false);
      assert.throws(() => Schema.decodeUnknownSync(schema)(input));
    }
  }
});

test("extending a mutually recursive struct preserves inherited fields", () => {
  const inputA: ExtendedA = { label: "root", b: { a: { b: {} } } };
  assert.deepEqual(Schema.decodeUnknownSync(ExtendedA)(inputA), inputA);
  assert.deepEqual(Schema.encodeSync(ExtendedA)(inputA), inputA);
  assert.equal(Schema.is(ExtendedA)({ label: "root", b: 42 }), false);
  assert.equal(Schema.is(ExtendedA)({ label: "root", b: { a: 42 } }), false);
});

test("extending a suspended schema preserves inherited fields", () => {
  const inputB: ExtendedB = { label: "root", a: { b: { a: {} } } };
  assert.deepEqual(Schema.decodeUnknownSync(ExtendedB)(inputB), inputB);
  assert.deepEqual(Schema.encodeSync(ExtendedB)(inputB), inputB);
  assert.equal(Schema.is(ExtendedB)({ label: "root", a: 42 }), false);
  assert.equal(Schema.is(ExtendedB)({ label: "root", a: { b: 42 } }), false);
});

test("extension cycles work with the base declared first", () => {
  for (const schema of [ReversedA, ReversedB]) {
    const input = { a: { a: {} } };
    assert.deepEqual(Schema.decodeUnknownSync(schema)(input), input);
    assert.deepEqual(Schema.encodeSync(schema)(input), input);
    assert.equal(Schema.is(schema)({ a: 42 }), false);
  }
});

test("recursive generic objects can extend ordinary and generic bases", () => {
  const ordinary = RecursiveExtended(Schema.NumberFromString);
  const encoded: Schema.Codec.Encoded<typeof ordinary> = { label: "root", value: "1", next: { label: "leaf", value: "2" } };
  const decoded: Schema.Schema.Type<typeof ordinary> = { label: "root", value: 1, next: { label: "leaf", value: 2 } };
  assert.deepEqual(Schema.decodeUnknownSync(ordinary)(encoded), decoded);
  assert.deepEqual(Schema.encodeSync(ordinary)(decoded), encoded);
  // @ts-expect-error The inherited label remains required on recursive values.
  const missing: Schema.Schema.Type<typeof ordinary> = { label: "root", value: 1, next: { value: 2 } };
  assert.equal(Schema.is(ordinary)(missing), false);

  const generic = RecursiveGenericExtended(Schema.NumberFromString);
  const genericEncoded: Schema.Codec.Encoded<typeof generic> = { label: "root", inherited: "1", next: { label: "leaf", inherited: "2" } };
  const genericDecoded: Schema.Schema.Type<typeof generic> = { label: "root", inherited: 1, next: { label: "leaf", inherited: 2 } };
  assert.deepEqual(Schema.decodeUnknownSync(generic)(genericEncoded), genericDecoded);
  assert.deepEqual(Schema.encodeSync(generic)(genericDecoded), genericEncoded);
  // @ts-expect-error The local label overrides the inherited number type.
  const invalid: Schema.Schema.Type<typeof generic> = { label: 42, inherited: 1 };
  assert.equal(Schema.is(generic)(invalid), false);

  const concrete = RecursiveConcreteExtended(Schema.NumberFromString);
  const concreteEncoded = { label: 1, inherited: ["hello"], value: "2", next: { label: 3, inherited: [], value: "4" } };
  const concreteDecoded: Schema.Schema.Type<typeof concrete> = { label: 1, inherited: ["hello"], value: 2, next: { label: 3, inherited: [], value: 4 } };
  assert.deepEqual(Schema.decodeUnknownSync(concrete)(concreteEncoded), concreteDecoded);
  assert.deepEqual(Schema.encodeSync(concrete)(concreteDecoded), concreteEncoded);
});

test("recursive field annotations retain inherited fields and allow overrides", () => {
  const input: InheritedChild = { id: "root", label: "child", a: { b: { id: "nested" } } };
  assert.deepEqual(Schema.decodeUnknownSync(InheritedChild)(input), input);
  assert.deepEqual(Schema.encodeSync(InheritedChild)(input), input);
  // @ts-expect-error Inherited fields must remain required in the inferred type.
  const missing: InheritedChild = { label: "child" };
  assert.equal(Schema.is(InheritedChild)(missing), false);
  const override: InheritedOverride = { id: 42 };
  assert.deepEqual(Schema.decodeUnknownSync(InheritedOverride)(override), override);
  // @ts-expect-error Local fields override inherited fields.
  const invalid: InheritedOverride = { id: "old" };
  assert.equal(Schema.is(InheritedOverride)(invalid), false);
});
