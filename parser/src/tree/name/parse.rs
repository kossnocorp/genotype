use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTName;

impl TryFrom<Pair<'_, Rule>> for GTName {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(GTName(pair.as_str().to_string()))
    }
}
