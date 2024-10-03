use parser::Rule;
use pest::iterators::Pair;
use tree::{GTDescriptor, GTResolve};

use crate::*;

use super::GTTuple;

impl GTTuple {
    pub fn parse(
        pair: Pair<'_, Rule>,
        resolve: &mut GTResolve,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut tuple = GTTuple {
            descriptors: vec![],
        };

        for pair in pair.into_inner() {
            let descriptor = GTDescriptor::parse(pair, resolve)?;
            tuple.descriptors.push(descriptor);
        }

        Ok(tuple)
    }
}
