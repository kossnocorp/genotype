use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTArray;

impl TryFrom<Pair<'_, Rule>> for GTArray {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let pair = pair.into_inner().next().unwrap(); // [TODO]
        let descriptor = pair.try_into()?;
        Ok(GTArray { descriptor })
    }
}
