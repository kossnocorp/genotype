use crate::{use_reference::RSUseReference, RSDependency};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSUse {
    pub reference: RSUseReference,
    pub dependency: RSDependency,
}
