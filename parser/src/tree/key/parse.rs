use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTKey;

impl GTKey {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GTKey(pair.as_str().into())
    }
}
