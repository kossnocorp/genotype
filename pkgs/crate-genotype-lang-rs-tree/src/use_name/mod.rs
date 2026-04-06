use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RsUseName {
    Name(#[visit] RsIdentifier),
    Alias(#[visit] RsIdentifier, #[visit] RsIdentifier),
}

impl RsUseName {
    pub fn name(&self) -> &RsIdentifier {
        match self {
            RsUseName::Name(name) => name,
            RsUseName::Alias(_, name) => name,
        }
    }

    pub fn original_name(&self) -> &RsIdentifier {
        match self {
            RsUseName::Name(name) => name,
            RsUseName::Alias(name, _) => name,
        }
    }
}

impl GtlImportRefName for RsUseName {}

impl From<&str> for RsUseName {
    fn from(str: &str) -> Self {
        RsUseName::Name(str.into())
    }
}

impl From<RsIdentifier> for RsUseName {
    fn from(identifier: RsIdentifier) -> Self {
        RsUseName::Name(identifier)
    }
}
