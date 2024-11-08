use crate::identifier::RSIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSReference {
    pub identifier: RSIdentifier,
}

impl RSReference {
    pub fn new(identifier: RSIdentifier) -> Self {
        RSReference { identifier }
    }
}
