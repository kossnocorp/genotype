use crate::GTSpan;

use super::{property::GTProperty, GTExtension};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTObject {
    pub span: GTSpan,
    pub extensions: Vec<GTExtension>,
    pub properties: Vec<GTProperty>,
}
