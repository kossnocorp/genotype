use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYDescriptor {
    List(Box<PYList>),
    Literal(PYLiteral),
    Primitive(PYPrimitive),
    Reference(PYReference),
    Tuple(PYTuple),
    Union(PYUnion),
    Dict(Box<PYDict>),
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

impl From<PYTuple> for PYDescriptor {
    fn from(tuple: PYTuple) -> Self {
        PYDescriptor::Tuple(tuple)
    }
}

impl From<PYList> for PYDescriptor {
    fn from(list: PYList) -> Self {
        PYDescriptor::List(Box::new(list))
    }
}

impl From<PYDict> for PYDescriptor {
    fn from(dict: PYDict) -> Self {
        PYDescriptor::Dict(Box::new(dict))
    }
}

impl From<PYLiteral> for PYDescriptor {
    fn from(literal: PYLiteral) -> Self {
        PYDescriptor::Literal(literal)
    }
}
