import type { LiteralBag, Response } from "genotype-test-literal-fields-types";

const _success: Response = {
  status: "success",
  value: "hello",
};

// @ts-expect-error
_success.status = "failure";

const _failure: Response = {
  status: "failure",
  error: "boom",
};

// @ts-expect-error
_failure.status = "success";

const _nope: Response = {
  // @ts-expect-error
  status: "nope",
  wrong: "uh oh",
};

const bag: LiteralBag = {
  kind: "demo",
  enabled: true,
  code: 200,
  empty: null,
};
