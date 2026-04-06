import {
  SelfRefArray,
  SelfRefObject,
  SelfRefTuple,
} from "genotype-test-self-refs-ts-zod";
import assert from "node:assert/strict";

// direct self-reference object case
type _SelfRefObject = SelfRefObject;

// recursive array alias case should accept finite base value
const arrayInput: SelfRefArray = [];
const arrayParsed = SelfRefArray.parse(arrayInput);
assert.deepEqual(arrayParsed, arrayInput);

// recursive tuple case should accept finite base value
const tupleInput: SelfRefTuple = null;
const tupleParsed = SelfRefTuple.parse(tupleInput);
assert.equal(tupleParsed, tupleInput);
