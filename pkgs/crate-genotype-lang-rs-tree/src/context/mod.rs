use crate::prelude::internal::*;

mod convert;
pub use convert::*;

#[cfg(test)]
pub mod mock;

#[derive(PartialEq)]
pub enum RSContextRenderDeriveTypeMode {
    Struct,
    UnionEnum,
}

#[derive(PartialEq)]
pub enum RSContextRenderDeriveSerdeMode {
    Serde,
    Litty,
}

pub trait RSConvertContextMockable {
    fn render_derive(
        &self,
        _type_mode: RSContextRenderDeriveTypeMode,
        _serde_mode: RSContextRenderDeriveSerdeMode,
    ) -> String {
        String::new()
    }
}

pub trait RSConvertContextConstraint:
    RSConvertContextMockable
    + GtlConvertContext<DependencyIdent = RSDependencyIdent, DependencyRef = RSIdentifier>
{
}
