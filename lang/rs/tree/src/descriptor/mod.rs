use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDescriptor {
    List(Box<RSList>),
    Literal(RSLiteral),
    Primitive(RSPrimitive),
    Reference(RSReference),
    Tuple(RSTuple),
    Union(RSUnion),
    Dict(Box<RSDict>),
    Any(RSAny),
}

impl From<RSPrimitive> for RSDescriptor {
    fn from(primitive: RSPrimitive) -> Self {
        RSDescriptor::Primitive(primitive)
    }
}

impl From<RSReference> for RSDescriptor {
    fn from(reference: RSReference) -> Self {
        RSDescriptor::Reference(reference)
    }
}

impl From<RSUnion> for RSDescriptor {
    fn from(union: RSUnion) -> Self {
        RSDescriptor::Union(union)
    }
}

impl From<RSTuple> for RSDescriptor {
    fn from(tuple: RSTuple) -> Self {
        RSDescriptor::Tuple(tuple)
    }
}

impl From<RSList> for RSDescriptor {
    fn from(list: RSList) -> Self {
        RSDescriptor::List(Box::new(list))
    }
}

impl From<RSDict> for RSDescriptor {
    fn from(dict: RSDict) -> Self {
        RSDescriptor::Dict(Box::new(dict))
    }
}

impl From<RSLiteral> for RSDescriptor {
    fn from(literal: RSLiteral) -> Self {
        RSDescriptor::Literal(literal)
    }
}

impl From<RSAny> for RSDescriptor {
    fn from(any: RSAny) -> Self {
        RSDescriptor::Any(any)
    }
}
