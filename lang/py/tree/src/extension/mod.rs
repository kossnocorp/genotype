use crate::PYReference;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYExtension {
    pub reference: PYReference,
}

impl From<&str> for PYExtension {
    fn from(str: &str) -> Self {
        PYExtension {
            reference: str.into(),
        }
    }
}
