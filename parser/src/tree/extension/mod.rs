use super::GTReference;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTExtension {
    pub reference: GTReference,
}
