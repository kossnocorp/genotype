use pest::iterators::Pair;

use crate::{
    parser::Rule,
    tree::{GTDescriptor, GTResolve},
};

use super::GTArray;

impl GTArray {
    pub fn parse(
        pair: Pair<'_, Rule>,
        resolve: &mut GTResolve,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let pair = pair.into_inner().next().unwrap(); // [TODO]
        let descriptor = GTDescriptor::parse(pair, resolve)?;
        Ok(GTArray { descriptor })
    }
}
