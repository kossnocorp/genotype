use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDescriptor {
    Enum(Box<RSEnum>),
    Vec(Box<RSVec>),
    Primitive(RSPrimitive),
    Reference(RSReference),
    InlineUse(RSInlineUse),
    Tuple(RSTuple),
    HashMap(Box<RSHashMap>),
    Option(Box<RSOption>),
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

impl From<RSHashMap> for RSDescriptor {
    fn from(hash_map: RSHashMap) -> Self {
        RSDescriptor::HashMap(Box::new(hash_map))
    }
}

impl From<RSOption> for RSDescriptor {
    fn from(option: RSOption) -> Self {
        RSDescriptor::Option(Box::new(option))
    }
}
