use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTIdentifier;

impl From<Pair<'_, Rule>> for GTIdentifier {
    fn from(pair: Pair<'_, Rule>) -> Self {
        GTIdentifier(pair.as_str().into())
    }
}
