use crate::prelude::internal::*;

#[cfg(test)]
pub use insta::{assert_debug_snapshot, assert_ron_snapshot, assert_snapshot};
#[cfg(test)]
pub use pretty_assertions::{
    assert_eq as assert_equal, assert_ne as assert_not_equal, assert_str_eq as assert_str_equal,
};

mod parser;
pub use parser::*;

pub struct GtFactory {}

impl GtFactory {
    pub fn literal_boolean(value: bool) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Boolean(value),
        }
    }

    pub fn literal_integer(value: i64) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Integer(value),
        }
    }

    pub fn literal_float(value: f64) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Float(value),
        }
    }

    pub fn literal_string(value: &str) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::String(value.into()),
        }
    }

    pub fn literal_null() -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Null,
        }
    }

    pub fn primitive_boolean() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Boolean,
        }
    }

    pub fn primitive_string() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::String,
        }
    }

    pub fn primitive_number() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Number,
        }
    }

    pub fn primitive_i8() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int8,
        }
    }

    pub fn primitive_i16() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int16,
        }
    }

    pub fn primitive_i32() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int32,
        }
    }

    pub fn primitive_i64() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int64,
        }
    }

    pub fn primitive_i128() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int128,
        }
    }

    pub fn primitive_isize() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntSize,
        }
    }

    pub fn primitive_u8() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU8,
        }
    }

    pub fn primitive_u16() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU16,
        }
    }

    pub fn primitive_u32() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU32,
        }
    }

    pub fn primitive_u64() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU64,
        }
    }

    pub fn primitive_u128() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU128,
        }
    }

    pub fn primitive_usize() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntUSize,
        }
    }

    pub fn primitive_f32() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Float32,
        }
    }

    pub fn primitive_f64() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Float64,
        }
    }
}
