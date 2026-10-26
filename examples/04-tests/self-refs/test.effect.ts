import { Schema } from "effect";
import { SelfRefArray, SelfRefObject, SelfRefTuple } from "genotype-test-self-refs-ts-effect";
import assert from "node:assert/strict";

// direct self-reference object case
type _SelfRefObject = SelfRefObject;

// recursive array alias case should accept finite base value
const arrayInput: SelfRefArray = [];
const arrayParsed = Schema.decodeUnknownSync(SelfRefArray)(arrayInput);
assert.deepEqual(arrayParsed, arrayInput);

// recursive tuple case should accept finite base value
const tupleInput: SelfRefTuple = null;
const tupleParsed = Schema.decodeUnknownSync(SelfRefTuple)(tupleInput);
assert.equal(tupleParsed, tupleInput);
