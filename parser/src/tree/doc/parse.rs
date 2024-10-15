use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTDoc;

impl GTDoc {
    pub fn concat(&self, pair: Pair<'_, Rule>) -> Self {
        let added_span = pair.as_span();
        let span = (self.0 .0, added_span.end()).into();
        GTDoc(span, format!("{}\n{}", self.1, pair.as_str()))
    }
}

impl From<Pair<'_, Rule>> for GTDoc {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let span = pair.as_span().into();
        GTDoc(span, pair.as_str().into())
    }
}
