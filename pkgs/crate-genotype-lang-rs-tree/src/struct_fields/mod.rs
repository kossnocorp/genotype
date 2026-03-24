use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RSStructFields {
    Unit,
    Newtype(#[visit] Vec<RSDescriptor>),
    Resolved(#[visit] Vec<RSField>),
    // Unresolved fields state. It represents fields extended struct fields as Rust has no
    // inheritance and the fields yet to be copied from the parent struct.
    Unresolved(GTSpan, #[visit] Vec<RSReference>, #[visit] Vec<RSField>),
}

impl From<Vec<RSField>> for RSStructFields {
    fn from(fields: Vec<RSField>) -> Self {
        RSStructFields::Resolved(fields)
    }
}
