use crate::prelude::internal::*;

#[cfg(test)]
pub mod mock;

pub trait TSConvertContextMockable {}

pub trait TSConvertContextConstraint:
    TSConvertContextMockable
    + GtlConvertContext<DependencyIdent = TSDependencyIdent, DependencyRef = TSIdentifier>
{
}
