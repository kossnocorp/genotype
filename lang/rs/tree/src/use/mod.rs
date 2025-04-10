use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSUse {
    pub reference: RSUseReference,
    pub dependency: RSDependency,
}
