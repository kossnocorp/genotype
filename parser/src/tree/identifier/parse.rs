use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTIdentifier;

impl GTIdentifier {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GTIdentifier::new(pair.as_span().into(), pair.as_str().into())
    }
}
