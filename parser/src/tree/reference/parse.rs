use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTReference;

impl TryFrom<Pair<'_, Rule>> for GTReference {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(GTReference::Unresolved(pair.as_str().into()))
    }
}
