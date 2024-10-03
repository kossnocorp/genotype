use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTDoc;

impl GTDoc {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GTDoc(pair.as_str().into())
    }

    pub fn concat(&self, pair: Pair<'_, Rule>) -> Self {
        GTDoc(format!("{}\n{}", self.0, pair.as_str()))
    }
}
