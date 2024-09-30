use crate::{array::TSArray, name::TSName, primitive::TSPrimitive, tuple::TSTuple, union::TSUnion};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSTypeDescriptor {
    Primitive(TSPrimitive),
    Name(TSName),
    Union(TSUnion),
    Array(Box<TSArray>),
    Tuple(Box<TSTuple>),
}
