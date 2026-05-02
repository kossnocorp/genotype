# Litty

Literally adds literals to Rust enum variants, struct fields, and unit structs.

Litty extends [Serde](https://serde.rs) and allows you to represent TypeScript-like literal types, bridging the gap between the type systems.

## Installation

Add `litty` to your `Cargo.toml`:

```toml
[dependencies]
litty = "0.3.1"
```

Or install using `cargo add`:

```bash
cargo add litty
```

## Usage

Litty is a Serde extension and provides a set of drop-in `Serialize` and `Deserialize` derive macro replacements (`SerializeLiterals` and `DeserializeLiterals`), allowing you to add literal fields to a struct or mark certain enum variants as literals.

It also provides a `literal` attribute macro that can be used on unit structs and enum variants to mark them as literals.

### Struct Fields

With `SerializeLiterals` and `DeserializeLiterals` derive macros, you can add literal fields to a struct:

```rs
use litty::{DeserializeLiterals, SerializeLiterals};

#[derive(SerializeLiterals, DeserializeLiterals)]
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

### Enum Variants

You can also mark enum variants as literals using `SerializeLiterals` and `DeserializeLiterals`:

```rs
use litty::{DeserializeLiterals, SerializeLiterals};

#[derive(SerializeLiterals, DeserializeLiterals)]
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

### Combined Serialization and Deserialization for Structs and Enums

Litty also provides the `Literals` derive macro that combines both `SerializeLiterals` and `DeserializeLiterals` into one:

```rs
use litty::{Literals};

#[derive(Literals)]
#[literals(v = 1, status = "ok")]
pub struct SuccessResponseV1 {
    pub message: String,
}

#[derive(Literals)]
pub enum Status {
    #[literal("ok")]
    Ok,
    #[literal("error")]
    Error,
}
```

### Unit Structs

You can also use `literal` on unit structs to create literal types:

```rs
use litty::{literal};

#[literal("ok")]
pub struct StatusOk;

#[literal("error")]
pub struct StatusError;
```

This directly maps to TypeScript types like this:

```ts
type StatusOk = "ok";

type StatusError = "error";
```

The `StatusOk` and `StatusError` unit structs will serialize to JSON strings `"ok"` and `"error"`, respectively.

#### Granular Serialization and Deserialization for Unit Structs

If you want to add serialization or deserialization to a unit struct without the other, you can use the `serialize_literal` or `deserialize_literal` attribute macros:

```rs
use litty::{serialize_literal, deserialize_literal};

#[serialize_literal("ok")]
pub struct StatusOk;

#[deserialize_literal("error")]
pub struct StatusError;
```

## Changelog

See [the changelog](./CHANGELOG.md).

## License

[MIT © Sasha Koss](https://koss.nocorp.me/mit/)
