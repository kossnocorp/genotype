use crate::prelude::internal::*;

mod convert;
pub use convert::*;

#[cfg(test)]
pub mod mock;

pub trait TsConvertContextMockable {}

pub trait TsConvertContextConstraint:
    TsConvertContextMockable
    + GtlConvertContext<DependencyIdent = TsDependencyIdent, DependencyRef = TsIdentifier>
{
}
