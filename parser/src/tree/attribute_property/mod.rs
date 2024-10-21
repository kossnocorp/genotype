use crate::GTSpan;

use super::{GTAttributeKey, GTAttributeValue};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAttributeProperty {
    pub span: GTSpan,
    pub name: GTAttributeKey,
    pub value: Option<GTAttributeValue>,
}
