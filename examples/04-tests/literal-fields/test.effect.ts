import { Schema } from "effect";
import {
  Executor,
  ExecutorKind,
  Formatter,
  LiteralBag,
  RemoveFileRequest,
  Response,
} from "genotype-test-literal-fields-effect-types";
import assert from "node:assert/strict";

assert.deepEqual(
  Schema.decodeUnknownSync(Response)({
    status: "success",
    value: "hello",
  }),
  {
    status: "success",
    value: "hello",
  },
);

assert.deepEqual(
  Schema.decodeUnknownSync(Response)({
    status: "failure",
    error: "boom",
  }),
  {
    status: "failure",
    error: "boom",
  },
);

assert.deepEqual(
  Schema.decodeUnknownSync(LiteralBag)({
    kind: "demo",
    enabled: true,
    code: 200,
    empty: null,
  }),
  {
    kind: "demo",
    enabled: true,
    code: 200,
    empty: null,
  },
);

assert.deepEqual(
  Schema.decodeUnknownSync(RemoveFileRequest)({
    requestType: "remove-file",
    request_kind: "file-operation",
    filePath: "src/main.type",
    retry_count: 2,
  }),
  {
    requestType: "remove-file",
    request_kind: "file-operation",
    filePath: "src/main.type",
    retry_count: 2,
  },
);

assert.equal(Schema.decodeUnknownSync(ExecutorKind)("cargo"), "cargo");
assert.equal(Schema.decodeUnknownSync(ExecutorKind)("pnpm"), "pnpm");
assert.equal(Schema.decodeUnknownSync(ExecutorKind)("uv"), "uv");
assert.equal(Schema.is(ExecutorKind)("yarn"), false);

assert.deepEqual(
  Schema.decodeUnknownSync(Executor)({
    kind: "pnpm",
    cmd: "prettier",
  }),
  {
    kind: "pnpm",
    cmd: "prettier",
  },
);

assert.deepEqual(
  Schema.decodeUnknownSync(Executor)({
    kind: "uv",
    cmd: "ruff",
  }),
  {
    kind: "uv",
    cmd: "ruff",
  },
);

assert.equal(
  Schema.is(Executor)({
    kind: "yarn",
    cmd: "prettier",
  }),
  false,
);

assert.deepEqual(
  Schema.decodeUnknownSync(Formatter)({
    kind: "shell",
    cmd: "npm run format",
  }),
  {
    kind: "shell",
    cmd: "npm run format",
  },
);

assert.deepEqual(
  Schema.decodeUnknownSync(Formatter)({
    kind: "cargo",
    cmd: "fmt",
  }),
  {
    kind: "cargo",
    cmd: "fmt",
  },
);

assert.equal(
  Schema.is(Formatter)({
    kind: "other",
    cmd: "fmt",
  }),
  false,
);
