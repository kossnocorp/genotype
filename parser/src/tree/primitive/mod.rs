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

impl Into<GTDescriptor> for GTPrimitive {
    fn into(self) -> GTDescriptor {
        GTDescriptor::Primitive(self)
    }
}
