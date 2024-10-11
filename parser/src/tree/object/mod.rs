use super::{property::GTProperty, GTExtension};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTObject {
    pub extensions: Vec<GTExtension>,
    pub properties: Vec<GTProperty>,
}
