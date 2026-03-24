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
    DeserializeLiterals, Literals, SerializeLiterals, deserialize_literal, literal,
    serialize_literal,
};
