use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTPrimitive {
    pub span: GTSpan,
    pub kind: GTPrimitiveKind,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
}

impl Display for GTPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Into<GTDescriptor> for GTPrimitive {
    fn into(self) -> GTDescriptor {
        GTDescriptor::Primitive(self)
    }
}
