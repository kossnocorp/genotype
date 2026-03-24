use crate::prelude::internal::*;

mod convert;
pub use convert::*;

#[cfg(test)]
pub mod mock;

#[derive(PartialEq)]
pub enum RSContextRenderDeriveMode {
    Struct,
    UnionEnum,
}

pub trait RSConvertContextMockable {
    fn render_derive(&self, _mode: RSContextRenderDeriveMode) -> String {
        String::new()
    }
}

pub trait RSConvertContextConstraint:
    RSConvertContextMockable
    + GtlConvertContext<DependencyIdent = RSDependencyIdent, DependencyRef = RSIdentifier>
{
}
