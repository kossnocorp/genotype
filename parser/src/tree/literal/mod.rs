use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTLiteral {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    pub value: GTLiteralValue,
}

impl GTLiteral {
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}
