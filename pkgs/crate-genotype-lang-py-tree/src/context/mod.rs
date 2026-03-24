use crate::prelude::internal::*;

mod convert;
pub use convert::*;

pub trait PYConvertContextMockable {
    fn is_version(&self, version: PYVersion) -> bool;
}

pub trait PYConvertContextConstraint:
    PYConvertContextMockable
    + GtlConvertContext<DependencyIdent = PYDependencyIdent, DependencyRef = PYIdentifier>
{
}
