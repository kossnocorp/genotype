use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsImportName {
    Name(#[visit] TsIdentifier),
    Alias(#[visit] TsIdentifier, #[visit] TsIdentifier),
}

impl GtlImportRefName for TsImportName {}

impl From<&str> for TsImportName {
    fn from(str: &str) -> Self {
        TsImportName::Name(str.into())
    }
}
