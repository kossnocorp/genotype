use crate::prelude::internal::*;

mod convert;
pub use convert::*;

pub trait PyConvertContextMockable {
    fn is_version(&self, version: PyVersion) -> bool;
}

pub trait PyConvertContextConstraint:
    PyConvertContextMockable
    + GtlConvertContext<DependencyIdent = PyDependencyIdent, DependencyRef = PyIdentifier>
{
}
