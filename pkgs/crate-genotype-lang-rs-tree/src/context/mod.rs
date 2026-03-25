use crate::prelude::internal::*;

mod convert;
pub use convert::*;

#[cfg(test)]
pub mod mock;

#[derive(PartialEq)]
pub enum RsContextRenderDeriveTypeMode {
    Struct,
    UnionEnum,
}

#[derive(PartialEq)]
pub enum RsContextRenderDeriveSerdeMode {
    Serde,
    Litty,
}

pub trait RsConvertContextMockable {
    fn render_derive(
        &self,
        _type_mode: RsContextRenderDeriveTypeMode,
        _serde_mode: RsContextRenderDeriveSerdeMode,
    ) -> String {
        String::new()
    }
}

pub trait RsConvertContextConstraint:
    RsConvertContextMockable
    + GtlConvertContext<DependencyIdent = RsDependencyIdent, DependencyRef = RsIdentifier>
{
}
