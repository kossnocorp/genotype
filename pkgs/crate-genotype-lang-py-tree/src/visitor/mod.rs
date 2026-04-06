use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, TraverseMut, Visitor};

#[visitor(
    nodes(
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
    ),
    mut_trait = PyVisitorMut
)]
pub struct PyVisitor;
