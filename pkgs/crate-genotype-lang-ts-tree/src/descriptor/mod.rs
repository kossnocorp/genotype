use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TSDescriptor {
    Array(#[visit] Box<TSArray>),
    InlineImport(#[visit] TSInlineImport),
    Intersection(#[visit] TSIntersection),
    Literal(#[visit] TSLiteral),
    Object(#[visit] TSObject),
    Primitive(#[visit] TSPrimitive),
    Reference(#[visit] TSReference),
    Tuple(#[visit] TSTuple),
    Union(#[visit] TSUnion),
    Record(#[visit] Box<TSRecord>),
    Any(#[visit] TSAny),
}

impl From<&str> for TSDescriptor {
    fn from(str: &str) -> Self {
        TSDescriptor::Reference(str.into())
    }
}

impl From<TSAny> for TSDescriptor {
    fn from(any: TSAny) -> Self {
        TSDescriptor::Any(any)
    }
}

impl From<TSIntersection> for TSDescriptor {
    fn from(intersection: TSIntersection) -> Self {
        TSDescriptor::Intersection(intersection)
    }
}

impl From<TSObject> for TSDescriptor {
    fn from(object: TSObject) -> Self {
        TSDescriptor::Object(object)
    }
}

impl From<TSPrimitive> for TSDescriptor {
    fn from(primitive: TSPrimitive) -> Self {
        TSDescriptor::Primitive(primitive)
    }
}

impl From<TSReference> for TSDescriptor {
    fn from(reference: TSReference) -> Self {
        TSDescriptor::Reference(reference)
    }
}

impl From<TSUnion> for TSDescriptor {
    fn from(union: TSUnion) -> Self {
        TSDescriptor::Union(union)
    }
}
