use crate::{
    array::TSArray, inline_import::TSInlineImport, name::TSName, object::TSObject,
    primitive::TSPrimitive, tuple::TSTuple, union::TSUnion,
};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSTypeDescriptor {
    Primitive(TSPrimitive),
    Name(TSName),
    Union(TSUnion),
    Object(Box<TSObject>),
    Array(Box<TSArray>),
    Tuple(Box<TSTuple>),
    InlineImport(TSInlineImport),
}
