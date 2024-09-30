use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTTuple;

impl TryFrom<Pair<'_, Rule>> for GTTuple {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut tuple = GTTuple {
            descriptors: vec![],
        };

        for pair in pair.into_inner() {
            let descriptor = pair.try_into()?;
            tuple.descriptors.push(descriptor);
        }

        Ok(tuple)
    }
}
