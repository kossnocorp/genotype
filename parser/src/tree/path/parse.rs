use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTPath;

impl TryFrom<Pair<'_, Rule>> for GTPath {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(GTPath(pair.as_str().to_string()))
    }
}
