use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTKey;

impl From<Pair<'_, Rule>> for GTKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        GTKey(pair.as_str().into())
    }
}
