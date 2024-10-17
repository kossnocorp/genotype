use crate::GTSpan;

use super::{property::GTProperty, GTExtension, GTObjectName};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTObject {
    pub span: GTSpan,
    pub name: GTObjectName,
    pub extensions: Vec<GTExtension>,
    pub properties: Vec<GTProperty>,
}
