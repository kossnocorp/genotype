import type { Status, StatusCode } from "genotype-test-literal-aliases-types";

const _success: Status = {
  status: "success",
  code: 200,
};
const _failure: Status = {
  status: "failure",
  code: 500,
};

const _codeOk: StatusCode = 200;
const _codeErr: StatusCode = 500;
