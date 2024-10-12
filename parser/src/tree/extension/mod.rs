use super::GTReference;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTExtension {
    pub reference: GTReference,
}

impl From<&str> for GTExtension {
    fn from(str: &str) -> Self {
        GTExtension {
            reference: str.into(),
        }
    }
}
