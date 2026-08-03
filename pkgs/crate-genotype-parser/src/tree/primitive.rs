use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtPrimitive {
    pub span: GtSpan,
    pub kind: GtPrimitiveKind,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
}

impl Display for GtPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl From<GtPrimitive> for GtDescriptor {
    fn from(val: GtPrimitive) -> Self {
        GtDescriptor::Primitive(val)
    }
}
