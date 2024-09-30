use super::descriptor::{parse_descriptor, GTDescriptor};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone)]
pub struct GTTuple {
    pub descriptors: Vec<GTDescriptor>,
}

pub fn parse_tuple(pair: Pair<'_, Rule>) -> Result<GTTuple, Box<dyn std::error::Error>> {
    let mut tuple = GTTuple {
        descriptors: vec![],
    };

    for pair in pair.into_inner() {
        let descriptor = parse_descriptor(pair)?;
        tuple.descriptors.push(descriptor);
    }

    Ok(tuple)
}
