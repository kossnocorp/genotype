import { Response, ResponsePair, ResponseString } from "genotype-test-generics-effect-types";
import assert from "node:assert";
import { Schema } from "effect";

const _responseStrOk: ResponseString = {
  status: "success",
  value: "Hello, world!",
};

assert(Schema.is(ResponseString)(_responseStrOk));

const _responseStrErr: ResponseString = {
  status: "success",
  // @ts-expect-error
  value: 123,
};

assert(!Schema.is(ResponseString)(_responseStrErr));

const ResponseNumber = Response(Schema.Number);

type ResponseNumber = Schema.Schema.Type<typeof ResponseNumber>;

const _responseOk: ResponseNumber = {
  status: "success",
  value: 42,
};

assert(Schema.is(ResponseNumber)(_responseOk));

const _responseErr: ResponseNumber = {
  status: "success",
  // @ts-expect-error
  value: "42",
};

assert(!Schema.is(ResponseNumber)(_responseErr));

const _responseFailure: ResponseNumber = {
  status: "failure",
  error: "Something went wrong",
};

assert(Schema.is(ResponseNumber)(_responseFailure));

const _responsePairOk: ResponsePair = {
  status: "success",
  value: {
    left: "left value",
    right: 123,
  },
};

assert(Schema.is(ResponsePair)(_responsePairOk));

const _responsePairErr: ResponsePair = {
  status: "success",
  value: {
    // @ts-expect-error
    left: 123,
    // @ts-expect-error
    right: "not a number",
  },
};

assert(!Schema.is(ResponsePair)(_responsePairErr));

// Schema factories preserve encoded types as well as decoded types.
const ResponseNumberFromString = Response(Schema.NumberFromString);
assert.deepEqual(
  Schema.decodeUnknownSync(ResponseNumberFromString)({ status: "success", value: "42" }),
  { status: "success", value: 42 },
);
assert.deepEqual(
  Schema.encodeSync(ResponseNumberFromString)({ status: "success", value: 42 }),
  { status: "success", value: "42" },
);
