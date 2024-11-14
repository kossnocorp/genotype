use genotype_parser::GTSpan;

use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSStructFields {
    Resolved(Vec<RSProperty>),
    Unresolved(GTSpan, Vec<RSReference>, Vec<RSProperty>),
}

impl From<Vec<RSProperty>> for RSStructFields {
    fn from(properties: Vec<RSProperty>) -> Self {
        RSStructFields::Resolved(properties)
    }
}
