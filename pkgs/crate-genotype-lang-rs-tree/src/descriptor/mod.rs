use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RsDescriptor {
    Enum(#[visit] Box<RsEnum>),
    Vec(#[visit] Box<RsVec>),
    Primitive(#[visit] RsPrimitive),
    Reference(#[visit] RsReference),
    InlineUse(#[visit] RsInlineUse),
    Tuple(#[visit] RsTuple),
    Map(#[visit] Box<RsMap>),
    Option(#[visit] Box<RsOption>),
    Any(#[visit] RsAny),
}

impl From<RsEnum> for RsDescriptor {
    fn from(r#enum: RsEnum) -> Self {
        RsDescriptor::Enum(Box::new(r#enum))
    }
}

impl From<RsPrimitive> for RsDescriptor {
    fn from(primitive: RsPrimitive) -> Self {
        RsDescriptor::Primitive(primitive)
    }
}

impl From<RsReference> for RsDescriptor {
    fn from(reference: RsReference) -> Self {
        RsDescriptor::Reference(reference)
    }
}

impl From<RsInlineUse> for RsDescriptor {
    fn from(inline_use: RsInlineUse) -> Self {
        RsDescriptor::InlineUse(inline_use)
    }
}

impl From<RsTuple> for RsDescriptor {
    fn from(tuple: RsTuple) -> Self {
        RsDescriptor::Tuple(tuple)
    }
}

impl From<RsVec> for RsDescriptor {
    fn from(list: RsVec) -> Self {
        RsDescriptor::Vec(Box::new(list))
    }
}

impl From<RsMap> for RsDescriptor {
    fn from(map: RsMap) -> Self {
        RsDescriptor::Map(Box::new(map))
    }
}

impl From<RsOption> for RsDescriptor {
    fn from(option: RsOption) -> Self {
        RsDescriptor::Option(Box::new(option))
    }
}

impl From<RsAny> for RsDescriptor {
    fn from(any: RsAny) -> Self {
        RsDescriptor::Any(any)
    }
}
