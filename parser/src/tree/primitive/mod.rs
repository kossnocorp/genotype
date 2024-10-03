use super::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTPrimitive {
    Boolean,
    String,
    Int,
    Float,
}

impl Into<GTDescriptor> for GTPrimitive {
    fn into(self) -> GTDescriptor {
        GTDescriptor::Primitive(self)
    }
}
