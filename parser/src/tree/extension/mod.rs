use super::{GTIdentifier, GTReference};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTExtension {
    pub reference: GTReference,
}

impl From<GTReference> for GTExtension {
    fn from(reference: GTReference) -> Self {
        GTExtension { reference }
    }
}

impl From<GTIdentifier> for GTExtension {
    fn from(idenfitier: GTIdentifier) -> Self {
        GTExtension {
            reference: idenfitier.into(),
        }
    }
}
