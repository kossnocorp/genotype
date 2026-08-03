use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtGenericArgument {
    pub span: GtSpan,
    #[visit]
    pub descriptor: GtDescriptor,
}

impl<Type: Into<GtDescriptor>> From<Type> for GtGenericArgument {
    fn from(descriptor: Type) -> Self {
        let descriptor = descriptor.into();
        Self {
            span: descriptor.span(),
            descriptor,
        }
    }
}
