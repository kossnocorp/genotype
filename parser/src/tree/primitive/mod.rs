use std::fmt::Display;

use crate::diagnostic::span::GTSpan;

use super::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTPrimitive {
    Boolean(GTSpan),
    String(GTSpan),
    Int(GTSpan),
    Float(GTSpan),
    Null(GTSpan),
}

impl Display for GTPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GTPrimitive::Boolean(_) => write!(f, "bool"),
            GTPrimitive::String(_) => write!(f, "str"),
            GTPrimitive::Int(_) => write!(f, "int"),
            GTPrimitive::Float(_) => write!(f, "float"),
            GTPrimitive::Null(_) => write!(f, "null"),
        }
    }
}

impl Into<GTDescriptor> for GTPrimitive {
    fn into(self) -> GTDescriptor {
        GTDescriptor::Primitive(self)
    }
}
