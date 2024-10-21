use crate::GTSpan;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAttributeName {
    pub span: GTSpan,
    pub name: String,
}
