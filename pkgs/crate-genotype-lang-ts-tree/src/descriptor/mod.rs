use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsDescriptor {
    Array(#[visit] Box<TsArray>),
    InlineImport(#[visit] TsInlineImport),
    Intersection(#[visit] TsIntersection),
    Literal(#[visit] TsLiteral),
    Object(#[visit] TsObject),
    Primitive(#[visit] TsPrimitive),
    Reference(#[visit] TsReference),
    Tuple(#[visit] TsTuple),
    Union(#[visit] TsUnion),
    Record(#[visit] Box<TsRecord>),
    Any(#[visit] TsAny),
}

impl From<&str> for TsDescriptor {
    fn from(str: &str) -> Self {
        TsDescriptor::Reference(str.into())
    }
}

impl From<TsAny> for TsDescriptor {
    fn from(any: TsAny) -> Self {
        TsDescriptor::Any(any)
    }
}

impl From<TsIntersection> for TsDescriptor {
    fn from(intersection: TsIntersection) -> Self {
        TsDescriptor::Intersection(intersection)
    }
}

impl From<TsObject> for TsDescriptor {
    fn from(object: TsObject) -> Self {
        TsDescriptor::Object(object)
    }
}

impl From<TsPrimitive> for TsDescriptor {
    fn from(primitive: TsPrimitive) -> Self {
        TsDescriptor::Primitive(primitive)
    }
}

impl From<TsReference> for TsDescriptor {
    fn from(reference: TsReference) -> Self {
        TsDescriptor::Reference(reference)
    }
}

impl From<TsUnion> for TsDescriptor {
    fn from(union: TsUnion) -> Self {
        TsDescriptor::Union(union)
    }
}
