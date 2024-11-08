use crate::RSReference;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSExtension {
    pub reference: RSReference,
}

impl RSExtension {
    pub fn new(reference: RSReference) -> Self {
        RSExtension { reference }
    }
}

impl From<RSReference> for RSExtension {
    fn from(reference: RSReference) -> Self {
        RSExtension::new(reference)
    }
}
