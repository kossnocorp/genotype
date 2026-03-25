use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, Visitor};

#[visitor(
    PyAlias,
    PyAny,
    PyClass,
    PyDefinition,
    PyDependencyIdent,
    PyDescriptor,
    PyDict,
    PyDictKey,
    PyDoc,
    PyEmbedDefinition,
    PyExtension,
    PyIdentifier,
    PyImport,
    PyImportName,
    PyImportReference,
    PyKey,
    PyList,
    PyLiteral,
    PyModule,
    PyPath,
    PyPrimitive,
    PyProperty,
    PyReference,
    PyTuple,
    PyUnion,
    PyNewtype
)]
pub struct PyVisitor;
