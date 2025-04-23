use crate::prelude::internal::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
pub use mock::*;

pub trait PYConvertContextMockable {
    fn is_version(&self, version: PYVersion) -> bool;
}

pub trait PYConvertContextConstraint:
    PYConvertContextMockable
    + GtlConvertContext<DependencyIdent = PYDependencyIdent, DependencyRef = PYIdentifier>
{
}
