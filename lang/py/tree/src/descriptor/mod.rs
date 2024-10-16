use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYDescriptor {
    List(Box<PYList>),
    Literal(PYLiteral),
    Object(PYObject),
    Primitive(PYPrimitive),
    Reference(PYReference),
    Tuple(PYTuple),
    Union(PYUnion),
}

impl From<&str> for PYDescriptor {
    fn from(str: &str) -> Self {
        PYDescriptor::Reference(str.into())
    }
}

impl From<PYObject> for PYDescriptor {
    fn from(object: PYObject) -> Self {
        PYDescriptor::Object(object)
    }
}

impl From<PYPrimitive> for PYDescriptor {
    fn from(primitive: PYPrimitive) -> Self {
        PYDescriptor::Primitive(primitive)
    }
}

impl From<PYReference> for PYDescriptor {
    fn from(reference: PYReference) -> Self {
        PYDescriptor::Reference(reference)
    }
}

impl From<PYUnion> for PYDescriptor {
    fn from(union: PYUnion) -> Self {
        PYDescriptor::Union(union)
    }
}
