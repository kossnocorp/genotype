use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RsStructFields {
    Unit,
    Newtype(#[visit] Vec<RsDescriptor>),
    Resolved(#[visit] Vec<RsField>),
    // Unresolved fields state. It represents fields extended struct fields as Rust has no
    // inheritance and the fields yet to be copied from the parent struct.
    Unresolved(GtSpan, #[visit] Vec<RsReference>, #[visit] Vec<RsField>),
}

impl From<Vec<RsField>> for RsStructFields {
    fn from(fields: Vec<RsField>) -> Self {
        RsStructFields::Resolved(fields)
    }
}
