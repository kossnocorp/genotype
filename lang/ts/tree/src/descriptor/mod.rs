use crate::{
    array::TSArray, inline_import::TSInlineImport, object::TSObject, primitive::TSPrimitive,
    reference::TSReference, tuple::TSTuple, union::TSUnion,
};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDescriptor {
    Array(Box<TSArray>),
    InlineImport(TSInlineImport),
    Object(TSObject),
    Primitive(TSPrimitive),
    Reference(TSReference),
    Tuple(TSTuple),
    Union(TSUnion),
}
