use genotype_parser::GTSpan;

use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSStructFields {
    Tuple(Vec<RSDescriptor>),
    Resolved(Vec<RSField>),
    Unresolved(GTSpan, Vec<RSReference>, Vec<RSField>),
}

impl From<Vec<RSField>> for RSStructFields {
    fn from(fields: Vec<RSField>) -> Self {
        RSStructFields::Resolved(fields)
    }
}
