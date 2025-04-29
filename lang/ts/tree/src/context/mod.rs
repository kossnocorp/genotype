use crate::prelude::internal::*;

mod convert;
pub use convert::*;

#[cfg(test)]
pub mod mock;

pub trait TSConvertContextMockable {}

pub trait TSConvertContextConstraint:
    TSConvertContextMockable
    + GtlConvertContext<DependencyIdent = TSDependencyIdent, DependencyRef = TSIdentifier>
{
}
