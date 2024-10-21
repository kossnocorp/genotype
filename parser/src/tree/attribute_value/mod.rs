use super::GTLiteral;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTAttributeValue {
    Literal(GTLiteral),
}
