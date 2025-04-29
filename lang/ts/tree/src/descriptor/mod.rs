use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDescriptor {
    Array(Box<TSArray>),
    InlineImport(TSInlineImport),
    Intersection(TSIntersection),
    Literal(TSLiteral),
    Object(TSObject),
    Primitive(TSPrimitive),
    Reference(TSReference),
    Tuple(TSTuple),
    Union(TSUnion),
    Record(Box<TSRecord>),
    Any(TSAny),
}

impl From<&str> for TSDescriptor {
    fn from(str: &str) -> Self {
        TSDescriptor::Reference(str.into())
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
