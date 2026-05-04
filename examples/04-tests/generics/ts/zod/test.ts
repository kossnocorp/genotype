import { Response, ResponsePair, ResponseString } from "genotype-test-generics-zod-types";
import assert from "node:assert";
import { z } from "zod";

const _responseStrOk: ResponseString = {
  status: "success",
  value: "Hello, world!",
};

assert(ResponseString.safeParse(_responseStrOk).success);

const _responseStrErr: ResponseString = {
  status: "success",
  // @ts-expect-error
  value: 123,
};

assert(!ResponseString.safeParse(_responseStrErr).success);

const ResponseNumber = Response(z.number());

type ResponseNumber = z.infer<typeof ResponseNumber>;

const _responseOk: ResponseNumber = {
  status: "success",
  value: 42,
};

assert(ResponseNumber.safeParse(_responseOk).success);

const _responseErr: ResponseNumber = {
  status: "success",
  // @ts-expect-error
  value: "42",
};

assert(!ResponseNumber.safeParse(_responseErr).success);

const _responseFailure: ResponseNumber = {
  status: "failure",
  error: "Something went wrong",
};

assert(ResponseNumber.safeParse(_responseFailure).success);

const _responsePairOk: ResponsePair = {
  status: "success",
  value: {
    left: "left value",
    right: 123,
  },
};

assert(ResponsePair.safeParse(_responsePairOk).success);

const _responsePairErr: ResponsePair = {
  status: "success",
  value: {
    // @ts-expect-error
    left: 123,
    // @ts-expect-error
    right: "not a number",
  },
};

assert(!ResponsePair.safeParse(_responsePairErr).success);
