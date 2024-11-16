use crate::{path::RSPath, use_reference::RSUseReference, RSDependency};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSUse {
    // [TODO] Probably can get rid of it in favor of dependency
    pub path: RSPath,
    pub reference: RSUseReference,
    pub dependency: RSDependency,
}
