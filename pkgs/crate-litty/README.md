# Litty

Litty is a [Serde](https://serde.rs) extension that _literally_ adds TypeScript-like literals to Rust enum variants, struct fields, and unit structs.

## Installation

Add `litty` to your `Cargo.toml`:

```toml
[dependencies]
litty = "0.5.0"
```

Or install it using `cargo add`:

```bash
cargo add litty
```

## Usage

There are three kinds of literals you can add literals to:

- [Struct fields](#struct-fields) via `serde_literals` attribute.
- [Enum variants](#enum-variants) via `serde_literals` attribute.
- [Unit structs](#unit-structs) via `serde_literal` attribute.

> [!IMPORTANT]
> Make sure to use Litty attributes `serde_literals` and `serde_literal`
> **before** the Serde derive macros `Serialize` and `Deserialize`.

> [!NOTE]
> Litty requires just one of the Serde derive macros to be present. You can derive only `Serialize` or only `Deserialize` and Litty will add corresponding literal serialization or deserialization support.

### Struct Fields

Use `serde_literals` attribute to add literal fields to a struct:

```rs
use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[serde_literals]
#[derive(Serialize, Deserialize)]
#[literals(v = 1, status = "ok")]
pub struct SuccessResponseV1 {
    pub message: String,
}
```

It directly maps to a TypeScript type like this:

```ts
interface SuccessResponseV1 {
  v: 1;
  status: "ok";
  message: string;
}
```

The struct will serialize to JSON like this:

```json
{
  "v": 1,
  "status": "ok",
  "message": "Hello, world!"
}
```

#### Renaming Literal Fields

Use `rename` to serialize a literal field under a different name:

```rs
use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[serde_literals]
#[derive(Serialize, Deserialize)]
#[literals(request_type("remove-file", rename = "requestType"))]
pub struct RemoveFileRequest {
    pub file_path: String,
}
```

This directly maps to a TypeScript type like this:

```ts
interface RemoveFileRequest {
  requestType: "remove-file";
  file_path: string;
}
```

The struct will serialize to JSON like this:

```json
{
  "requestType": "remove-file",
  "file_path": "src/main.type"
}
```

### Enum Variants

Use `serde_literals` attribute to make enum variants literal types:

```rs
use litty::serde_literals;
use serde::{Deserialize, Serialize};

#[serde_literals]
#[derive(Serialize, Deserialize)]
pub enum Status {
    #[literal("ok")]
    Ok,
    #[literal("error")]
    Error,
}
```

It directly maps to a TypeScript type like this:

```ts
type Status = "ok" | "error";
```

The `Status::Ok` and `Status::Error` variants will serialize to JSON strings `"ok"` and `"error"`, respectively.

### Unit Structs

Use `serde_literal` attribute to make unit structs literal types:

```rs
use litty::serde_literal;
use serde::{Deserialize, Serialize};

#[serde_literal("ok")]
#[derive(Serialize, Deserialize)]
pub struct StatusOk;

#[serde_literal("error")]
#[derive(Serialize, Deserialize)]
pub struct StatusError;
```

This directly maps to TypeScript types like this:

```ts
type StatusOk = "ok";

type StatusError = "error";
```

The `StatusOk` and `StatusError` unit structs will serialize to JSON strings `"ok"` and `"error"`, respectively.

## Changelog

See [the changelog](./CHANGELOG.md).

## License

[MIT © Sasha Koss](https://koss.nocorp.me/mit/)
