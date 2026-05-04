import { Response, ResponsePair, ResponseString } from "genotype-test-generics-alias-types";

const _responseStrOk: ResponseString = {
  status: "success",
  value: "Hello, world!",
};

const _responseStrErr: ResponseString = {
  status: "success",
  // @ts-expect-error
  value: 123,
};

const _responseOk: Response<number> = {
  status: "success",
  value: 42,
};

const _responseErr: Response<number> = {
  status: "success",
  // @ts-expect-error
  value: "42",
};

const _responseFailure: Response<number> = {
  status: "failure",
  error: "Something went wrong",
};

const _responsePairOk: ResponsePair = {
  status: "success",
  value: {
    left: "left value",
    right: 123,
  },
};

const _responsePairErr: ResponsePair = {
  status: "success",
  value: {
    // @ts-expect-error
    left: 123,
    // @ts-expect-error
    right: "not a number",
  },
};
