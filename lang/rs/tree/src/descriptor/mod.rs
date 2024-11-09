use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDescriptor {
    List(Box<RSList>),
    Literal(RSLiteral),
    Primitive(RSPrimitive),
    Reference(RSReference),
    InlineUse(RSInlineUse),
    Tuple(RSTuple),
    Union(RSUnion),
    HashMap(Box<RSHashMap>),
    Any(RSAny),
    Option(Box<RSOption>),
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

impl From<RSInlineUse> for RSDescriptor {
    fn from(inline_use: RSInlineUse) -> Self {
        RSDescriptor::InlineUse(inline_use)
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

impl From<RSHashMap> for RSDescriptor {
    fn from(hash_map: RSHashMap) -> Self {
        RSDescriptor::HashMap(Box::new(hash_map))
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

impl From<RSOption> for RSDescriptor {
    fn from(option: RSOption) -> Self {
        RSDescriptor::Option(Box::new(option))
    }
}
