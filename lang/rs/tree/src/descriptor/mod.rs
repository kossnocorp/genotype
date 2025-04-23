use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDescriptor {
    Enum(Box<RSEnum>),
    Vec(Box<RSVec>),
    Primitive(RSPrimitive),
    Reference(RSReference),
    InlineUse(RSInlineUse),
    Tuple(RSTuple),
    Map(Box<RSMap>),
    Option(Box<RSOption>),
    Any(RSAny),
}

impl From<RSEnum> for RSDescriptor {
    fn from(r#enum: RSEnum) -> Self {
        RSDescriptor::Enum(Box::new(r#enum))
    }
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

impl From<RSTuple> for RSDescriptor {
    fn from(tuple: RSTuple) -> Self {
        RSDescriptor::Tuple(tuple)
    }
}

impl From<RSVec> for RSDescriptor {
    fn from(list: RSVec) -> Self {
        RSDescriptor::Vec(Box::new(list))
    }
}

impl From<RSMap> for RSDescriptor {
    fn from(map: RSMap) -> Self {
        RSDescriptor::Map(Box::new(map))
    }
}

impl From<RSOption> for RSDescriptor {
    fn from(option: RSOption) -> Self {
        RSDescriptor::Option(Box::new(option))
    }
}

impl From<RSAny> for RSDescriptor {
    fn from(any: RSAny) -> Self {
        RSDescriptor::Any(any)
    }
}
