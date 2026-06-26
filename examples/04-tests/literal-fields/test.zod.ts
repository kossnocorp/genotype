import {
  Executor,
  ExecutorKind,
  Formatter,
  LiteralBag,
  RemoveFileRequest,
  Response,
} from "genotype-test-literal-fields-zod-types";
import assert from "node:assert/strict";

assert.deepEqual(
  Response.parse({
    status: "success",
    value: "hello",
  }),
  {
    status: "success",
    value: "hello",
  },
);

assert.deepEqual(
  Response.parse({
    status: "failure",
    error: "boom",
  }),
  {
    status: "failure",
    error: "boom",
  },
);

assert.deepEqual(
  LiteralBag.parse({
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
  RemoveFileRequest.parse({
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

assert.equal(ExecutorKind.parse("cargo"), "cargo");
assert.equal(ExecutorKind.parse("pnpm"), "pnpm");
assert.equal(ExecutorKind.parse("uv"), "uv");
assert.equal(ExecutorKind.safeParse("yarn").success, false);

assert.deepEqual(
  Executor.parse({
    kind: "pnpm",
    cmd: "prettier",
  }),
  {
    kind: "pnpm",
    cmd: "prettier",
  },
);

assert.deepEqual(
  Executor.parse({
    kind: "uv",
    cmd: "ruff",
  }),
  {
    kind: "uv",
    cmd: "ruff",
  },
);

assert.equal(
  Executor.safeParse({
    kind: "yarn",
    cmd: "prettier",
  }).success,
  false,
);

assert.deepEqual(
  Formatter.parse({
    kind: "shell",
    cmd: "npm run format",
  }),
  {
    kind: "shell",
    cmd: "npm run format",
  },
);

assert.deepEqual(
  Formatter.parse({
    kind: "cargo",
    cmd: "fmt",
  }),
  {
    kind: "cargo",
    cmd: "fmt",
  },
);

assert.equal(
  Formatter.safeParse({
    kind: "other",
    cmd: "fmt",
  }).success,
  false,
);
