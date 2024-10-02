use crate::{
    array::TSArray, inline_import::TSInlineImport, object::TSObject, primitive::TSPrimitive,
    reference::TSReference, tuple::TSTuple, union::TSUnion,
};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDescriptor {
    Primitive(TSPrimitive),
    Reference(TSReference),
    Union(TSUnion),
    Object(Box<TSObject>),
    Array(Box<TSArray>),
    Tuple(Box<TSTuple>),
    InlineImport(TSInlineImport),
}
