mod null;
pub use null::*;

mod str;
pub use str::*;

mod bool;
pub use bool::*;

mod int;
pub use int::*;

mod float;
pub use float::*;

mod r#enum;

pub use litty_macro::{
    deserialize_literal, literal, serde_literal, serde_literals, serialize_literal,
};

#[deprecated(note = "use #[serde_literals] with #[derive(Serialize, Deserialize)] instead")]
pub use litty_macro::Literals;

#[deprecated(note = "use #[serde_literals] with #[derive(Serialize)] instead")]
pub use litty_macro::SerializeLiterals;

#[deprecated(note = "use #[serde_literals] with #[derive(Deserialize)] instead")]
pub use litty_macro::DeserializeLiterals;
