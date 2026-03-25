use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PyDescriptor {
    List(#[visit] Box<PyList>),
    Literal(#[visit] PyLiteral),
    Primitive(#[visit] PyPrimitive),
    Reference(#[visit] PyReference),
    Tuple(#[visit] PyTuple),
    Union(#[visit] PyUnion),
    Dict(#[visit] Box<PyDict>),
    Any(#[visit] PyAny),
}

impl From<PyPrimitive> for PyDescriptor {
    fn from(primitive: PyPrimitive) -> Self {
        PyDescriptor::Primitive(primitive)
    }
}

impl From<PyReference> for PyDescriptor {
    fn from(reference: PyReference) -> Self {
        PyDescriptor::Reference(reference)
    }
}

impl From<PyUnion> for PyDescriptor {
    fn from(union: PyUnion) -> Self {
        PyDescriptor::Union(union)
    }
}

impl From<PyTuple> for PyDescriptor {
    fn from(tuple: PyTuple) -> Self {
        PyDescriptor::Tuple(tuple)
    }
}

impl From<PyList> for PyDescriptor {
    fn from(list: PyList) -> Self {
        PyDescriptor::List(Box::new(list))
    }
}

impl From<PyDict> for PyDescriptor {
    fn from(dict: PyDict) -> Self {
        PyDescriptor::Dict(Box::new(dict))
    }
}

impl From<PyLiteral> for PyDescriptor {
    fn from(literal: PyLiteral) -> Self {
        PyDescriptor::Literal(literal)
    }
}

impl From<PyAny> for PyDescriptor {
    fn from(any: PyAny) -> Self {
        PyDescriptor::Any(any)
    }
}
