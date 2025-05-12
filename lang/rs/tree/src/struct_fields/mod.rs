use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSStructFields {
    Unit,
    Newtype(Vec<RSDescriptor>),
    Resolved(Vec<RSField>),
    // Unresolved fields state. It represents fields extended struct fields as Rust has no
    // inheritance and the fields yet to be copied from the parent struct.
    Unresolved(GTSpan, Vec<RSReference>, Vec<RSField>),
}

impl From<Vec<RSField>> for RSStructFields {
    fn from(fields: Vec<RSField>) -> Self {
        RSStructFields::Resolved(fields)
    }
}
